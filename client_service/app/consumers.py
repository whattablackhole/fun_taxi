from channels.generic.websocket import AsyncWebsocketConsumer
from channels.layers import channel_layers
from .channel_manager import ChannelManager
import json



channel_manager = ChannelManager()

class ClientConsumer(AsyncWebsocketConsumer):
    async def connect(self):
        self.user_id = self.scope["user"].id
        await self.accept()
        channel_manager.add_user(self.user_id, self.channel_name)
    
    async def disconnect(self, close_code):
        channel_manager.remove_user(self.user_id)

    async def receive(self, text_data):
        text_data_json = json.loads(text_data)
        message = text_data_json['message']
        pass

    async def transportation_request_apply(self, event):
        message = event['message']
        await self.send(text_data=json.dumps({
            'message': message
        }))

