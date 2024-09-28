from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
import redis
from .kafka_consumer import consume_kafka
from .schemas import DriverSearchArea, Endpoints
import threading
import psycopg2
import os

db = redis.Redis(host='localhost', port=6379, db=0)

current_dir = os.path.dirname(__file__)

sql_file_path = os.path.join(current_dir, "queries", "shortest_path.sql")

map_db = psycopg2.connect(
    dbname='map_db',
    user='user',
    password='password',
    host='localhost',
    port='5444'
)

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

threading.Thread(target=consume_kafka, args=(redis.Redis(host='localhost', port=6379, db=0),), daemon=True).start()

@app.post("/nearestdrivers")
async def get_nearest_drivers(area: DriverSearchArea):
    nearby_drivers = db.georadius(
        'driver_positions',
        area.lon,
        area.lat,
        area.radius,
        unit='km',
        withcoord=True
    )
    results = []
    for driver_id, position in nearby_drivers:
        bearing = db.hget('driver_bearings', driver_id)
        results.append({
            "driver_id": driver_id,
            "position": {
                "lng": position[0],
                "lat": position[1]
            },
            "bearing": bearing
        })

    return results


@app.post("/shortest_path")
async def get_shortest_path(endpoints: Endpoints):
    try:
        cur = map_db.cursor()
        with open(sql_file_path, "r") as file:
            sql_query = file.read()
            sql_query = sql_query.replace("lon", str(endpoints.start_lon), 1)
            sql_query = sql_query.replace("lat", str(endpoints.start_lat), 1)
            sql_query = sql_query.replace("lon", str(endpoints.start_lon), 1)
            sql_query = sql_query.replace("lat", str(endpoints.start_lat), 1)
            sql_query = sql_query.replace("lon", str(endpoints.end_lon), 1)
            sql_query = sql_query.replace("lat", str(endpoints.end_lat), 1)
            sql_query = sql_query.replace("lon", str(endpoints.end_lon), 1)
            sql_query = sql_query.replace("lat", str(endpoints.end_lat), 1)
            cur.execute(sql_query)
            result = cur.fetchone()

            return result[0]
    except Exception as e:
        return JSONResponse(status_code=500, content={"error": str(e)})



