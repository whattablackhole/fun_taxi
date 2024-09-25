from fastapi import HTTPException
from sqlalchemy.orm import Session
from . import models, schemas, auth

def get_user_by_email(db: Session, email: str):
    return db.query(models.User).filter(models.User.email == email).one_or_none()

def get_user_by_username(db: Session, username: str):
    return db.query(models.User).filter(models.User.username == username).one_or_none()

def create_user(db: Session, user: schemas.UserCreate):
    hashed_password = auth.get_password_hash(user.password)

    db_user = models.User(email=user.email, hashed_password=hashed_password, username=user.username)

    default_role = db.query(models.Role).filter(models.Role.name == "user").first()

    if default_role:
        db_user.roles.append(default_role)

    db.add(db_user)
    db.commit()
    db.refresh(db_user)

    return db_user

def add_new_role(db: Session, role: schemas.RoleCreate):
    role = models.Role(name = role.name)
    db.add(role)
    db.commit()
    db.refresh(role)
    return role


def add_user_role(db: Session, user: models.User, role: str):
    role = db.query(models.Role).filter(models.Role.name == role).first()
    
    if role is None:
        raise HTTPException(status_code=400, detail="Role is not found")
    
    user.roles.append(role)

    db.add(user)
    db.commit()
    db.refresh(user)
    return user
        

def add_driver_profile(db: Session, user: models.User, driver: models.Driver):
    if user.driver_profile:
        raise HTTPException(status_code=400, detail="User already has a driver profile")

    user.driver_profile = driver

    db.commit()

    db.refresh(user)

    return user.driver_profile


def create_driver(session: Session, user: models.User):
    has_driver_role = any(user_role.name == "driver" for user_role in user.roles)

    if has_driver_role:
        raise HTTPException(status_code=400, detail="User already has the driver role")

    if user.driver_profile:
        raise HTTPException(status_code=400, detail="User already has a driver profile")

    role = session.query(models.Role).filter(models.Role.name == "driver").first()

    if role is None:
        raise HTTPException(status_code=400, detail="Role has not found")

    user.roles.append(role)
    
    driver_profile = models.Driver()
    user.driver_profile = driver_profile
    
    session.commit()
    session.refresh(user)
    return user



