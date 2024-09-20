from confluent_kafka import Consumer
from channels.layers import get_channel_layer
import asyncio

async def consume_kafka():
    c = Consumer({ "bootstrap.servers": "localhost:9092", "group.id": "drivers_group"})
    c.subscribe(["transportation_request"])
    channel_layer = get_channel_layer()

    
    while (True):
        await asyncio.sleep(1)

        message = c.poll(0)

        if (message is None):
            continue

        if (message.error()):
            print(message.error())
            continue

        print("hello")

        value = message.value().decode('utf-8')
        print(value)

        await channel_layer.group_send('drivers_group', {
                "type": "transportation_request",
                "message": value
        })

        
         
            
        
