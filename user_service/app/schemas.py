from pydantic import BaseModel, EmailStr
from typing import Optional


class UserBase(BaseModel):
    email: EmailStr
    second_name: Optional[str] = None
    first_name: str

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
    token_types: str


class TokenData(BaseModel):
    type: str
    email: Optional[EmailStr] = None

class TokenRefresh(BaseModel):
    refresh_token: str

    