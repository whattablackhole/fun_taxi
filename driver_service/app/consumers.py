from channels.generic.websocket import AsyncWebsocketConsumer
import json

class DriverConsumer(AsyncWebsocketConsumer):
    async def connect(self):
        await self.accept()
        await self.channel_layer.group_add("drivers_group", self.channel_name)

    async def disconnect(self, close_code):
        await self.channel_layer.group_discard("drivers_group", self.channel_name)

    async def receive(self, text_data):
        text_data_json = json.loads(text_data)
        message = text_data_json['message']
        print(message)
    
    async def transportation_request(self, event):
        message = event["message"]
        await self.send(text_data=message)

