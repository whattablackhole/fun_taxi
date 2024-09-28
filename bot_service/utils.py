import re
from models import Position
import math

def toRadians(degrees):
        return degrees * math.pi / 180
    
def toDegrees(radians):
        return radians * 180 / math.pi

def haversine_distance(pos1: Position, pos2: Position) -> float:
    R = 6371000  # Radius of Earth in meters
    lat1, lon1 = pos1.lat, pos1.lng
    lat2, lon2 = pos2.lat, pos2.lng

    lat1_rad, lon1_rad = map(toRadians, [lat1, lon1])
    lat2_rad, lon2_rad = map(toRadians, [lat2, lon2])

    d_lat = lat2_rad - lat1_rad
    d_lon = lon2_rad - lon1_rad

    a = (math.sin(d_lat / 2) ** 2 +
         math.cos(lat1_rad) * math.cos(lat2_rad) * math.sin(d_lon / 2) ** 2)
    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))

    return R * c

def parse_point(point_text) -> Position:
    match = re.match(r'POINT\(([-\d\.]+) ([-\d\.]+)\)', point_text)
    if match:
        lng, lat = map(float, match.groups())
        return Position(lat=lat, lng=lng)
    else:
        raise ValueError("Invalid POINT format")



    