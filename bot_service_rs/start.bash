#!/bin/bash
kubectl port-forward svc/nginx-ingress-nginx-controller 8011:80 &
APP_ENV=kubernetes cargo run 