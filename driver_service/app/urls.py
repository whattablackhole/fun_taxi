from django.urls import path

from app.views import hello_world

urlpatterns = [
    path('hello/', hello_world, name='hello-world'),
]