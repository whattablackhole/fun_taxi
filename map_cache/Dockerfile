FROM openresty/openresty:1.21.4.1-0-alpine-fat AS nginx

COPY ./nginx.conf /usr/local/openresty/nginx/conf/nginx.conf

COPY ./conf.d/ /etc/nginx/conf.d/

RUN mkdir -p /var/cache/nginx/tiles \
    && chown -R nobody:nobody /var/cache/nginx/tiles