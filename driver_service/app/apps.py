from django.apps import AppConfig
import asyncio
from app.consume_kafka import consume_kafka

class AppConfig(AppConfig):
    default_auto_field = 'django.db.models.BigAutoField'
    name = 'app'
    
    def ready(self):
        loop = asyncio.get_event_loop()
        loop.create_task(consume_kafka())
