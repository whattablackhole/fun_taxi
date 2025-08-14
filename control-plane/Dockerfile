FROM golang:1.23.8-alpine AS builder

RUN apk add --no-cache git

WORKDIR /app

COPY go.mod go.sum ./

RUN go mod download

COPY . .

RUN go build -o xds-server .

FROM alpine:latest

RUN apk add --no-cache ca-certificates

WORKDIR /root/

COPY --from=builder /app/xds-server .

ENV CONSUL_HTTP_ADDRESS=http://consul:8500
ENV LISTENER_PORT=10000

EXPOSE 10000

CMD ["./xds-server"]