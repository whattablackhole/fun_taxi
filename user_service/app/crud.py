from sqlalchemy.orm import Session
from . import models, schemas, auth

def get_user_by_email(db: Session, email: str):
    return db.query(models.User).filter(models.User.email == email).one_or_none()

def get_user_by_username(db: Session, username: str):
    return db.query(models.User).filter(models.User.username == username).one_or_none()

def create_user(db: Session, user: schemas.UserCreate):
    hashed_password = auth.get_password_hash(user.password)

    db_user = models.User(email=user.email, hashed_password=hashed_password, username=user.username)

    db.add(db_user)
    
    db.commit()
    
    db.refresh(db_user)
    
    return db_user
