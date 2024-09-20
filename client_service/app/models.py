from django.db import models


# TODO: convert to data model
class Position(models.Model):
    def __init__(self, lat: float, lon: float):
        self.lat = lat
        self.lon = lon


class TransportationRequest(models.Model):
    user_id = models.CharField(max_length=255)
    start_point = models.JSONField()
    end_point = models.JSONField()