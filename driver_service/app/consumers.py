from channels.generic.websocket import AsyncWebsocketConsumer


class WSConsumer(AsyncWebsocketConsumer):
    def connect(self):
        self.accept()

    def disconnect():
        pass

    def receive(self, text_data=None, bytes_data=None):
        return super().receive(text_data, bytes_data)

