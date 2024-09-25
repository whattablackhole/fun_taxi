from sqlalchemy.orm import Session

from .models import Role

def seed_db(db: Session):
    user_role, created_user_role = get_or_create(db, Role, name="user")
    driver_role, created_driver_role = get_or_create(db, Role, name="driver")

    if created_user_role:
        print("User role created.")
    else:
        print("User role already exists.")

    if created_driver_role:
        print("Driver role created.")
    else:
        print("Driver role already exists.")


def get_or_create(db: Session, model, **kwargs):
    instance = db.query(model).filter_by(**kwargs).first()
    if instance:
        return instance, False 
    else:
        instance = model(**kwargs)
        db.add(instance)
        db.commit()
        return instance, True