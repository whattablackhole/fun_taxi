from django.http import HttpRequest
import jwt


def get_user_from_request(request: HttpRequest):
    token = request.headers.get('Authorization', '').split(' ')[1]
    try:
        user_data = validate_jwt_token(token)
    except:
        return None
    return user_data

def validate_jwt_token(token: str):
    secret = "my_secret_key"

    return jwt.decode(token, secret, ["HS256"])