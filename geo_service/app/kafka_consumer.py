from confluent_kafka import Consumer
import json
from redis import Redis


def consume_kafka(db: Redis):
    consumer = Consumer({
        'bootstrap.servers': 'localhost:9092',
        'group.id': 'your_group_id',
        'auto.offset.reset': 'earliest',
        'fetch.min.bytes': 50000,
        'fetch.max.bytes': 1048576,
    })

    consumer.subscribe(["driver_geo_position"])

    while True:
        messages = consumer.poll(timeout=1)
        if messages is None:
            continue
        print(messages.value())
        # TODO: batch the messages to db 
        if isinstance(messages, list):
            for message in messages:
                data = json.loads(message.value().decode('utf-8'))
                driver_id = data['driver_id']
                position = data['position']
                bearing = data['bearing']
                db.geoadd('driver_positions', [position['lng'], position['lat'], driver_id])
                db.hset('driver_bearings', driver_id, bearing)
                print(f"Updated position for driver {driver_id}: {position}")
        else:
            data = json.loads(messages.value().decode('utf-8'))
            driver_id = data['driver_id']
            position = data['position']
            bearing = data['bearing']
            db.hset('driver_bearings', driver_id, bearing)
            db.geoadd('driver_positions', [position['lng'], position['lat'], driver_id])
            print(f"Updated position for driver {driver_id}: {position}")
