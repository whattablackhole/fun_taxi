from django.urls import path
from app.views import transportation_request

urlpatterns = [
    path('trip_request/', transportation_request, name="trip_request")
]