#!/bin/bash

cat "$KAFKA_SETTINGS_PATH" | envsubst >> /app/kafka-settings.json

KAFKA_SETTINGS_PATH=/app/kafka-settings.json

sqlx migrate run && /app/application $1
