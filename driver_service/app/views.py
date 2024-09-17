from django.shortcuts import render
from rest_framework.response import Response
from rest_framework.decorators import api_view
from confluent_kafka import Producer

def delivery_report(err, msg):
    """Called once for each message produced to indicate delivery result."""
    if err is not None:
        print(f"Message delivery failed: {err}")
    else:
        print(f"Message delivered to {msg.topic()} [{msg.partition()}]")

@api_view(['GET'])
def hello_world(request):
    p = Producer({'bootstrap.servers': 'localhost:9092'})

    message = {"message": "Hello, World!"}
    message_str = str(message)
    p.produce('topic', message_str.encode('utf-8'), callback=delivery_report)
    p.flush()
    return Response({"message": "Hello, World!"})
