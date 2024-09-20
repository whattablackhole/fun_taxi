from rest_framework import serializers
from app.models import TransportationRequest

class PositionSerializer(serializers.Serializer):
    lat = serializers.FloatField()
    lon = serializers.FloatField()

class TransportationRequestSerializer(serializers.ModelSerializer):
    start_point = PositionSerializer()
    end_point = PositionSerializer()
    
    class Meta:
        model = TransportationRequest
        fields = ['user_id', 'start_point', 'end_point']

    def create(self, validated_data):
        start_point_data = validated_data.pop('start_point')
        end_point_data = validated_data.pop('end_point')

        transportation_request = TransportationRequest.objects.create(start_point=start_point_data, end_point=end_point_data, **validated_data)

        return transportation_request