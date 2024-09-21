from sqlalchemy.orm import Session
from . import models, schemas, auth

def get_user_by_email(db: Session, email: str):
    return db.query(models.User).filter(models.User.email == email).one()

def create_user(db: Session, user: schemas.UserCreate):
    hashed_password = auth.get_password_hash(user.password)

    db_user = models.User(email=user.password, hashed_password=hashed_password, first_name=user.first_name, last_name=user.second_name)

    db.add(db_user)
    
    db.commit()
    
    db.refresh(db_user)
    
    return db_user
