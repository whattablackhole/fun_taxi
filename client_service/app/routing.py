from django.urls import path
from . import consumers

websocket_urlpatterns = [
    path('ws/default/', consumers.ClientConsumer.as_asgi()),
]