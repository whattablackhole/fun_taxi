from django.core.management.base import BaseCommand
from channels.layers import get_channel_layer
from confluent_kafka import Consumer, KafkaError
import asyncio


class Command(BaseCommand):
    help = 'Consume Kafka messages'

    async def consume_kafka_messages(self):
        consumer = Consumer({
            'bootstrap.servers': 'localhost:9092',
            'group.id': 'your_group_id',
            'auto.offset.reset': 'earliest',
        })
        consumer.subscribe(['topic'])
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

            # Handle the message
            message = msg.value().decode('utf-8')

            print(f"Received message: {message}")
            # You might want to process or forward the message to WebSocket connections
            await channel_layer.group_send(
                'chat_group',  # This should match the group name in your WebSocket consumer
                {
                    'type': 'chat_message',  # This method will be handled in your WebSocket consumer
                    'message': message
                }
            )
            await asyncio.sleep(1)  # Avoid busy waiting

    def handle(self, *args, **kwargs):
        loop = asyncio.get_event_loop()
        loop.run_until_complete(self.consume_kafka_messages())