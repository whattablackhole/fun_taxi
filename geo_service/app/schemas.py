from pydantic import BaseModel


class DriverSearchArea(BaseModel):
    lat: float
    lon: float
    radius: int
