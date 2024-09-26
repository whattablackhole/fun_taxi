from fastapi import FastAPI
from contextlib import asynccontextmanager
import redis
from .kafka_consumer import consume_kafka
from asyncio import get_event_loop
from .schemas import DriverSearchArea
import threading

db = redis.Redis(host='localhost', port=6379, db=0)

app = FastAPI()


threading.Thread(target=consume_kafka, args=(db,), daemon=True).start()

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
        results.append({
            "driver_id": driver_id,
            "position": {
                "lng": position[0],
                "lat": position[1]
            }
        })

    return results

