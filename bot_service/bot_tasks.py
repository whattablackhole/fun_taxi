import asyncio
from dataclasses import dataclass
from celery import Celery
from confluent_kafka import Producer
import json
import math
import random

@dataclass
class Position:
    lat: float
    lng: float

@dataclass
class DriverGeoPosition:
    driver_id: int
    position: Position


app = Celery('bot_tasks')
app.config_from_object('celeryconfig')


class BotDriver():
    def __init__(self, init_positon: Position, driver_id: int):
        self.position = init_positon
        self.driver_id = driver_id
        self.producer = Producer({'bootstrap.servers': 'localhost:9092'})

    async def stream_geo_position(self):
        while(True):
            geo = {
                "driver_id": self.driver_id,
                "position": {"lat": self.position.lat, "lng": self.position.lng}
            }
            self.producer.produce('driver_geo_position', json.dumps(geo))
            self.producer.flush()
            self.move(0.01, 90)
            await asyncio.sleep(5)

    def move(self, distance_km, bearing_degrees):
        bearing_radians = math.radians(bearing_degrees)

        delta_lat = distance_km / 111

        delta_lng = distance_km / (111 * math.cos(math.radians(self.position.lat)))

        self.position.lat += delta_lat * math.cos(bearing_radians)
        self.position.lng += delta_lng * math.sin(bearing_radians)

def randomize_position(position: Position, radius_km: int) -> Position:
    EARTH_RADIUS_KM = 6371.0

    distance_km = random.uniform(0, radius_km)

    bearing = random.uniform(0, 2 * math.pi)

    angular_distance = distance_km / EARTH_RADIUS_KM

    lat_rad = math.radians(position.lat)
    lng_rad = math.radians(position.lng)

    new_lat_rad = math.asin(math.sin(lat_rad) * math.cos(angular_distance) +
                            math.cos(lat_rad) * math.sin(angular_distance) * math.cos(bearing))

    new_lng_rad = lng_rad + math.atan2(
        math.sin(bearing) * math.sin(angular_distance) * math.cos(lat_rad),
        math.cos(angular_distance) - math.sin(lat_rad) * math.sin(new_lat_rad)
    )

    new_lat = math.degrees(new_lat_rad)
    new_lng = math.degrees(new_lng_rad)

    return Position(lat=new_lat, lng=new_lng)


@app.task
def run_bot(driver_id):
    position = randomize_position(Position(52.1329457, 26.1130729), 5)
    bot = BotDriver(position, driver_id)
    asyncio.run(bot.stream_geo_position())