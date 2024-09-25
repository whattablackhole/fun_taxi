from django.urls import path

from app.views import hello_world, submit_order, list_orders

urlpatterns = [
    path('hello/', hello_world, name='hello-world'),
    path('list_orders/', list_orders, name='list_orders'),
    path('submit_order/', submit_order, name='create_order')
]