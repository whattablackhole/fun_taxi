from rest_framework import serializers
from .models import Order


class PositionSerializer(serializers.Serializer):
    lat = serializers.FloatField()
    lng = serializers.FloatField()

    def validate_lat(self, value):
        if value < -90 or value > 90:
            raise serializers.ValidationError("Latitude must be between -90 and 90 degrees.")
        return value

    def validate_lng(self, value):
        if value < -180 or value > 180:
            raise serializers.ValidationError("Longitude must be between -180 and 180 degrees.")
        return value


class OrderCreateSerializer(serializers.ModelSerializer):
    start_point = PositionSerializer()
    end_point = PositionSerializer()

    class Meta:
        model = Order
        fields = ['id', 'user_id', 'start_point', 'end_point']

    def validate(self, data):
        start_point = data.get('start_point')
        end_point = data.get('end_point')

        start_lat = start_point['lat']
        start_lng = start_point['lng']
        end_lat = end_point['lat']
        end_lng = end_point['lng']

        if start_lat == end_lat and start_lng == end_lng:
            raise serializers.ValidationError("Start point and end point cannot be the same.")

        return data


