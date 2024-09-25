import json
from rest_framework.decorators import api_view
from rest_framework.request import Request
from rest_framework.response import Response
from app.serializers import TransportationRequestSerializer
from confluent_kafka import Producer
import uuid

@api_view(['POST'])
def transportation_request(request: Request):
    serializer = TransportationRequestSerializer(data = request.data)

    if serializer.is_valid():
        serializer.data['id'] = uuid.uuid4
        transportation_request_json = json.dumps(serializer.data)

        p = Producer({'bootstrap.servers': 'localhost:9092'})

        p.produce('transportation_request', transportation_request_json)

        p.flush()
        return Response({"status": "Request sent to driver service"}, status=201)
    else:
        return Response(serializer.errors, status=400)