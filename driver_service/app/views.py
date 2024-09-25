from django.shortcuts import render
from rest_framework import status
from rest_framework.exceptions import ParseError,bad_request,ValidationError,PermissionDenied,AuthenticationFailed
from rest_framework.response import Response
from rest_framework.decorators import api_view
from confluent_kafka import Producer
from .serializers import OrderCreateSerializer
from .models import Order
from .services import user_service

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


@api_view(['POST'])
def create_order(request):
    # user = user_service.get_user_from_request(request)

    # if user is None:
    #     return bad_request(request, AuthenticationFailed())
    
    serializer = OrderCreateSerializer(data=request.data)

    if not serializer.is_valid():
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)
    
    order_data = serializer.validated_data

    exist = Order.objects.filter(id=order_data['id']).first()

    if exist is not None:
        return bad_request(request, PermissionDenied("order already exists"))


    order = Order(
        id = order_data['id'],
        user_id = order_data['user_id'],
        driver_id = 1,
        start_point = order_data['start_point'],
        end_point = order_data['end_point']
    )

    order_data['driver_id'] = 1

    order.save()


    p = Producer({'bootstrap.servers': 'localhost:9092'})

    p.produce('transportation_request_apply', order_data, callback=delivery_report)

    return Response(order_data, status=201)


@api_view(['GET'])
def list_orders(request):
    orders = Order.objects.all()
    serializer = OrderCreateSerializer(orders, many=True)
    return Response(serializer.data, status=status.HTTP_200_OK)