

resource "kubernetes_deployment" "weather-app" {
  metadata {
    name = "weather-app-deployment"
    labels = {
      app = "weather-app"
    }
  }

  spec {
    replicas = 1
    selector {
      match_labels = {
        app = "weather-app"
      }
    }
    template {
      metadata {
        labels = {
          app = "weather-app"
        }
      }
      spec {
        container {
          name  = "weather-app-container"
          image = "whattablackhole/weather-app"
          port {
            container_port = 8080
          }
        }
      }
    }
  }
  depends_on = [alicloud_cs_managed_kubernetes.k8s]
}

resource "kubernetes_service" "weather-app-service" {
  metadata {
    name = "weather-app"
  }
  spec {
    selector = {
      app = kubernetes_deployment.weather-app.metadata.0.labels.app
    }
    port {
      port        = 80
      target_port = 8080
      protocol    = "TCP"
    }
    type = "LoadBalancer"
  }
  depends_on = [alicloud_cs_managed_kubernetes.k8s]
}


# resource "kubernetes_ingress_v1" "weather-app_ingress" {
#   metadata {
#     name = "weather-app-ingress"
#     annotations = {
#       "nginx.ingress.kubernetes.io/rewrite-target" = "/"
#     }
#   }

#   spec {
#     ingress_class_name = "nginx"
#     rule {
#       http {
#         path {
#           path = "/"
#           path_type = "Prefix"
#           backend {
#             service {
#               name = kubernetes_service.weather-app_service.metadata.0.name
#               port {
#                 number = 80
#               }
#             }
#           }
#         }
#       }
#     }
#   }
# }