class ChannelManager:
    def __init__(self):
        self.user_channel_map = {}

    def add_user(self, user_id, channel_name):
        self.user_channel_map[user_id] = channel_name

    def remove_user(self, user_id):
        if user_id in self.user_channel_map:
            del self.user_channel_map[user_id]

    def get_channel_name(self, user_id):
        return self.user_channel_map.get(user_id)

channel_manager = ChannelManager()