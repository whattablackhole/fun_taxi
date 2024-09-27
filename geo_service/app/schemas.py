from pydantic import BaseModel


class DriverSearchArea(BaseModel):
    lat: float
    lon: float
    radius: int


class Endpoints(BaseModel):
    start_lat: float
    start_lon: float
    end_lat: float
    end_lon: float