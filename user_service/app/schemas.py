from pydantic import BaseModel, EmailStr
from typing import Optional


class UserBase(BaseModel):
    email: EmailStr
    username: str

class UserCreate(UserBase):
    password: str

class UserOut(UserBase):
    id: int
    is_active: int

    class Config:
        from_attributes = True

class Token(BaseModel):
    access_token: str
    refresh_token: str


class TokenData(BaseModel):
    type: str
    email: Optional[EmailStr] = None

class TokenRefresh(BaseModel):
    refresh_token: str

    