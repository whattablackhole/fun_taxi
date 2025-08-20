#!/bin/bash
set -e

BROKER="kafka:9092"
TOPIC="gps_driver_position"
PARTITIONS=3
REPLICATION=1
MAX_RETRIES=20
SLEEP_SEC=3
i=0

echo "Waiting for Kafka broker at $BROKER..."

while ! kafka-topics.sh --bootstrap-server $BROKER --list >/dev/null 2>&1; do
    i=$((i+1))
    echo "Kafka not ready yet... ($i/$MAX_RETRIES)"
    if [ $i -ge $MAX_RETRIES ]; then
        echo "Kafka broker did not become ready within $MAX_RETRIES attempts."
        echo "Continuing anyway — topic creation may fail if Kafka is not ready."
        break
    fi
    sleep $SLEEP_SEC
done

EXISTS=$(kafka-topics.sh --bootstrap-server $BROKER --list | grep "^$TOPIC$" || true)
if [ -z "$EXISTS" ]; then
    echo "Creating topic $TOPIC..."
    kafka-topics.sh --bootstrap-server $BROKER --create \
        --topic $TOPIC --partitions $PARTITIONS --replication-factor $REPLICATION || true
    echo "Topic $TOPIC created (or already exists)"
else
    echo "Topic $TOPIC already exists"
fi

exit 0