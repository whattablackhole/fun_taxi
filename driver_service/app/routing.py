from django.urls import path
from .consumers import WSConsumer


websocket_urlpatterns = [
    path('ws/default/', WSConsumer().as_asgi()),
]