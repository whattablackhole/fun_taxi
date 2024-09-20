from django.db import models


# TODO: convert to data model
class Position(models.Model):
    def __init__(self, lat: float, lng: float):
        self.lat = lat
        self.lon = lng


class TransportationRequest(models.Model):
    user_id = models.CharField(max_length=255)
    start_point = models.JSONField()
    end_point = models.JSONField()