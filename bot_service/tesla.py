import math
from utils import toDegrees, toRadians
from models import Position
class Tesla():
    def calculate_bearing(self, start: Position, end: Position):
        lon1, lat1 = start.lng, start.lat
        lon2, lat2 = end.lng, end.lat
        
        print(f"Start Position: {lon1}, {lat1}")
        print(f"End Position: {lon2}, {lat2}")

        d_lon = toRadians(lon2 - lon1)
        lat1_rad = toRadians(lat1)
        lat2_rad = toRadians(lat2)

        x = math.sin(d_lon) * math.cos(lat2_rad)
        y = math.cos(lat1_rad) * math.sin(lat2_rad) - math.sin(lat1_rad) * math.cos(lat2_rad) * math.cos(d_lon)

        bearing = math.atan2(x, y)
        return (toDegrees(bearing) + 360) % 360

    def move_by_bearing(self, curr_pos: Position, next_node: Position, speed: float) -> tuple[Position, float]:
        bearing = self.calculate_bearing(curr_pos, next_node)
        R = 6371000  # Radius of Earth in meters

        cur_lon, cur_lat = curr_pos.lng, curr_pos.lat
        lat1 = toRadians(cur_lat)
        lon1 = toRadians(cur_lon)

        d = speed / R  # Distance traveled in radians

        bearing_rad = toRadians(bearing)

        new_lat = math.asin(math.sin(lat1) * math.cos(d) + math.cos(lat1) * math.sin(d) * math.cos(bearing_rad))
        new_lon = lon1 + math.atan2(math.sin(bearing_rad) * math.sin(d) * math.cos(lat1), math.cos(d) - math.sin(lat1) * math.sin(new_lat))

        return (Position(lng=toDegrees(new_lon), lat=toDegrees(new_lat)), bearing)
