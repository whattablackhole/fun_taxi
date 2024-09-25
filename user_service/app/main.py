from contextlib import asynccontextmanager
from datetime import timedelta
from fastapi import FastAPI, Depends, HTTPException, status
from sqlalchemy.orm import Session, joinedload
from . import models, schemas, crud, auth, database, seed
from fastapi.security import OAuth2PasswordBearer, OAuth2PasswordRequestForm
from fastapi.middleware.cors import CORSMiddleware
from email_validator import validate_email, EmailNotValidError

@asynccontextmanager
async def lifespan(app: FastAPI):
    db = database.SessionLocal()
    seed.seed_db(db)
    db.close()

    yield

app = FastAPI(lifespan=lifespan)


app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

models.Base.metadata.create_all(bind=database.engine)

oauth2_scheme = OAuth2PasswordBearer(tokenUrl="token")

def get_current_user(token: str = Depends(oauth2_scheme), db: Session = Depends(database.get_db)):
    token_data = auth.decode_token(token)

    if token_data is None or token_data.type != "access":
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Could not validate credentials",
            headers={"WWW-Authenticate": "Bearer"}
        )
    user = crud.get_user_by_email(db, email=token_data.email)
    if user is None:
        raise HTTPException(status_code=404, detail="user not found")
    return user


@app.post("/register", response_model=schemas.UserOut)
def register(user: schemas.UserCreate, db: Session = Depends(database.get_db)):
    db_user = crud.get_user_by_email(db, email=user.email)
    if db_user:
        raise HTTPException(status_code=400, detail="Email already registered")
    created_user = crud.create_user(db=db, user=user)
    return created_user


@app.post("/token", response_model=schemas.AuthedUser)
def login(form_data: OAuth2PasswordRequestForm = Depends(), db: Session = Depends(database.get_db)):
    try:
        email = validate_email(form_data.username)
        user = crud.get_user_by_email(db, email)
    except EmailNotValidError:
        user = crud.get_user_by_username(db, form_data.username)
    if not user:
        raise HTTPException(status_code=400, detail="Incorrect email or password")
    
    if not auth.verify_password(form_data.password, user.hashed_password):
        raise HTTPException(status_code=400, detail="Incorrect email or password")
    
    access_token_expires = timedelta(minutes=auth.ACCESS_TOKEN_EXPIRE_MINUTES)
    refresh_token_expires = timedelta(minutes=auth.REFRESH_TOKEN_EXPIRE_MINUTES)
    

    access_token = auth.create_access_token(
        data={"email": user.email}, expires_delta=access_token_expires)
    
    refresh_token = auth.create_refresh_token(data = {"email": user.email}, expire_delta=refresh_token_expires)

    return {"access_token": access_token, "refresh_token": refresh_token, "user": user}



@app.post("/refresh", response_model=schemas.Token)
def refresh_token(token_refresh: schemas.TokenRefresh, db: Session = Depends(database.get_db)):
    token_data = auth.decode_token(token_refresh.refresh_token)
    if token_data is None or token_data.type != "refresh":
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Could not validate refresh token",
            headers={"WWW-Authenticate": "Bearer"},
        )
    user = crud.get_user_by_email(db, email=token_data.email)
    if user is None:
        raise HTTPException(status_code=404, detail="User not found")
    
    access_token_expires = timedelta(minutes=auth.ACCESS_TOKEN_EXPIRE_MINUTES)
    refresh_token_expires = timedelta(minutes=auth.REFRESH_TOKEN_EXPIRE_MINUTES)
    
    access_token = auth.create_access_token(
        data={"email": user.email}, expires_delta=access_token_expires
    )
    refresh_token = auth.create_refresh_token(
        data={"email": user.email}, expires_delta=refresh_token_expires
    )
    
    return {"access_token": access_token, "refresh_token": refresh_token, "token_type": "bearer"}


# TODO: protect by admin flag
@app.post("/roles", response_model=schemas.RoleOut)
def add_role(role: schemas.RoleCreate, session: Session = Depends(database.get_db)):
    return crud.add_new_role(session, role)

@app.get("/roles")
def get_roles(session: Session = Depends(database.get_db)):
    roles = session.query(models.Role).all()
    
    return roles


@app.get("/users")
def get_users_with_roles(session: Session = Depends(database.get_db)):
     return (
        session.query(models.User)
        .options(joinedload(models.User.roles))
        .all()
    )
        
        

@app.post("/users/role", response_model=schemas.UserOut)
def add_user_role(role: str, current_user: models.User = Depends(get_current_user), session: Session = Depends(database.get_db)):
    return crud.add_user_role(session, current_user, role)

@app.post("/drivers", response_model=schemas.UserOut)
def create_driver(current_user: models.User = Depends(get_current_user), session: Session = Depends(database.get_db)):
    return crud.create_driver(session, current_user)

@app.get("/users/me", response_model=schemas.UserOut)
def read_users_me(current_user: models.User = Depends(get_current_user)):
    return current_user