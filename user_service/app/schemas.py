from pydantic import BaseModel, EmailStr


class DriverOut(BaseModel):
    id: int

class UserBase(BaseModel):
    email: EmailStr
    username: str

class UserCreate(UserBase):
    password: str

class RoleOut(BaseModel):
    id: int
    name: str

    class Config:
        orm_mode = True


class UserOut(UserBase):
    id: int
    is_active: int
    roles: list[RoleOut]

    class Config:
        from_attributes = True

class Token(BaseModel):
    access_token: str
    refresh_token: str


class TokenData(BaseModel):
    type: str
    email: EmailStr | None = None

class TokenRefresh(BaseModel):
    refresh_token: str

class RoleCreate(BaseModel):
    name: str

