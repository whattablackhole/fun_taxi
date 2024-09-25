from django.db import models

class HelloWorld(models.Model):
    pass

class Order(models.Model):
  id = models.UUIDField(primary_key=True)
  driver_id = models.PositiveIntegerField()
  user_id =  models.PositiveIntegerField()
  start_point = models.JSONField()
  end_point = models.JSONField()
  is_active = models.BooleanField(default=True)