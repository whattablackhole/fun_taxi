import asyncio
from dataclasses import dataclass
from celery import Celery
from confluent_kafka import Producer
import json
import math
import random
from psycopg2 import connect
from tesla import Tesla
import httpx
from celery.signals import worker_process_init, worker_process_shutdown
from models import Position, FeatureCollection
from utils import haversine_distance, parse_point


app = Celery('bot_tasks')
app.config_from_object('celeryconfig')

db_conn = None

@worker_process_init.connect
def init_worker(**kwargs):
    global db_conn
    db_conn = connect(
        dbname='map_db',
        user='user',
        password='password',
        host='localhost',
        port='5444'
    )

@worker_process_shutdown.connect
def shutdown_worker(**kwargs):
    global db_conn
    if db_conn:
        db_conn.close()


class BotDriver():
    def __init__(self, init_pos: Position, driver_id: int):
        self.car = Tesla()
        self.producer = Producer({'bootstrap.servers': 'localhost:9092'})
        self.position = self.find_nearest_road_point(init_pos)
        self.destination = None
        self.driver_id = driver_id
        self.way_cursor = 0
        self.node_cursor = 0
        self.threshold = 25
        self.speed = 40
        self.bearing = 0
        self.route: FeatureCollection | None = None
    async def start_auto_pilot(self):
        while(True):
            while self.route == None or self.route.features == None or len(self.route.features) == 0:
                self.position = self.find_nearest_road_point(randomize_position(self.position, 5))
                self.destination = self.find_nearest_road_point(randomize_position(self.position, 10))
                self.route = await self.get_destination_route(self.position, self.destination)
                await asyncio.sleep(2)
                continue
            if (self.destination == self.position):
                    self.route = None
                    self.destination = None
                    self.way_cursor = 0
                    self.node_cursor = 0
                    continue
            self.move()
            self.stream_geo_position()
            await asyncio.sleep(2)


    
    async def get_destination_route(self, init_pos: Position, destination_pos: Position) -> FeatureCollection | None:
        payload = {
            "start_lon": init_pos.lng,
            "start_lat": init_pos.lat,
            "end_lon": destination_pos.lng,
            "end_lat": destination_pos.lat
        }

        async with httpx.AsyncClient() as client:
            response = await client.post("http://127.0.0.1:7777/shortest_path", json=payload)
        
        if response.status_code == 200:
            geo_data = FeatureCollection(**response.json())
            return geo_data

        else:
            print(f"Error: Received status code {response.status_code}")
            return None
        

    def stream_geo_position(self):
        geo = {
                "driver_id": self.driver_id,
                "position": {"lat": self.position.lat, "lng": self.position.lng},
                "bearing": self.bearing
            }
        self.producer.produce('driver_geo_position', json.dumps(geo))
        self.producer.flush()

    def move(self):
        geometry = self.route.features[self.way_cursor]['geometry']

        next_node = geometry['coordinates'][self.node_cursor]
        next_node_position = Position(lng=next_node[0], lat=next_node[1])

        distance = haversine_distance(self.position, next_node_position)
        
        if distance <= self.threshold:
            if len(geometry['coordinates']) - 1 > self.node_cursor:
                self.node_cursor += 1
                return
            elif len(self.route.features) - 1 > self.way_cursor:
                self.way_cursor += 1
                self.node_cursor = 0
                return
            else:
                self.position = self.destination
                return
            
        new_position, bearing = self.car.move_by_bearing(self.position, next_node_position, self.speed)
        self.position = new_position
        self.bearing = bearing



    def find_nearest_road_point(self, position: Position):
        query = f"""
        SELECT 
            ST_AsText(ST_Transform(ST_ClosestPoint(l.way, ST_Transform(ST_SetSRID(ST_MakePoint({position.lng}, {position.lat}), 4326), 3857)), 4326)) AS closest_point,
            ST_Distance(l.way, ST_Transform(ST_SetSRID(ST_MakePoint({position.lng}, {position.lat}), 4326), 3857)) AS distance
        FROM planet_osm_roads l
        WHERE l.highway IN ('motorway', 'trunk', 'primary', 'secondary', 'residential')
        ORDER BY distance
        LIMIT 1;
        """
        try:
            cur = db_conn.cursor()
            cur.execute(query)
            result = cur.fetchone()
            return parse_point(result[0])
        except Exception as e:
            print(e)
            raise e
        finally:
            if cur:
                cur.close()

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
    try:
        position = randomize_position(Position(52.1329457, 26.1130729), 5)
        bot = BotDriver(position, driver_id)
        asyncio.run(bot.start_auto_pilot())
    except Exception as e:
        print(e)

if __name__ == "__main__":
    run_bot(1)