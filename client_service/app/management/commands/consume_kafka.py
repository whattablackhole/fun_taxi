from django.core.management.base import BaseCommand
from channels.layers import get_channel_layer
from confluent_kafka import Consumer, KafkaError
import asyncio
from consumers import channel_manager
import json

class Command(BaseCommand):
    help = 'Consume Kafka messages'

    async def consume_kafka_messages(self):
        consumer = Consumer({
            'bootstrap.servers': 'localhost:9092',
            'group.id': 'your_group_id',
            'auto.offset.reset': 'earliest',
        })
        consumer.subscribe(['transportation_request_apply'])
        channel_layer = get_channel_layer()
        while True:
            msg = consumer.poll(timeout=1.0)
            if msg is None:
                continue
            if msg.error():
                if msg.error().code() == KafkaError._PARTITION_EOF:
                    continue
                else:
                    print(f"Error: {msg.error()}")
                    continue
            message = msg.value().decode('utf-8')
            message_data = json.loads(message)
            print(f"Received message: {message}")
            channel_name = channel_manager.get_channel_name(message_data['user_id'])
            if channel_name:
                await channel_layer.send(
                    channel_name,
                    {
                        'type': 'transportation_request_apply',
                        'message': message
                    }
                )
            await asyncio.sleep(1)  # Avoid busy waiting

    def handle(self, *args, **kwargs):
        loop = asyncio.get_event_loop()
        loop.run_until_complete(self.consume_kafka_messages())