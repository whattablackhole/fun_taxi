import asyncio
from dataclasses import dataclass
from celery import Celery
from confluent_kafka import Producer
import json
import math
import random
from psycopg2 import extensions, pool
import re


    

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

db_pool = pool.SimpleConnectionPool(
    1,  # Minimum number of connections
    10,  # Maximum number of connections
    dbname='map_db',
    user='user',
    password='password',
    host='localhost',
    port='5444'
)

def parse_point(point_text) -> Position:
    match = re.match(r'POINT\(([-\d\.]+) ([-\d\.]+)\)', point_text)
    if match:
        lon, lat = map(float, match.groups())
        return Position(lat, lon)
    else:
        raise ValueError("Invalid POINT format")
    

class BotDriver():
    def __init__(self, init_positon: Position, driver_id: int):
        self.position = self.find_nearest_road_point(init_positon)
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

    def find_nearest_road_point(self, position: Position):
        query = f"""
        SELECT 
            ST_AsText(ST_Transform(ST_ClosestPoint(l.way, ST_Transform(ST_SetSRID(ST_MakePoint({position.lng}, {position.lat}), 4326), 3857)), 4326)) AS closest_point,
            ST_Distance(l.way, ST_Transform(ST_SetSRID(ST_MakePoint({position.lng}, {position.lat}), 4326), 3857)) AS distance
        FROM planet_osm_line l
        WHERE l.highway IN ('motorway', 'trunk', 'primary', 'secondary', 'residential')
        ORDER BY distance
        LIMIT 1;
        """
        conn: extensions.connection = db_pool.getconn()
        try:
            cur = conn.cursor()
            cur.execute(query)
            result = cur.fetchone()
            return parse_point(result[0])
        finally:
            if cur:
                cur.close()
            db_pool.putconn(conn)

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