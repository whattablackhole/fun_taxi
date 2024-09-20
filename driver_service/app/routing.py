from django.urls import path
from .consumers import DriverConsumer


websocket_urlpatterns = [
    path('ws/default/', DriverConsumer().as_asgi()),
]