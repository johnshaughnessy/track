#!/usr/bin/env sh

HOST=$1

if [ -z "$HOST" ]; then
    echo "Please provide the HOST ip address or hostname as an argument."
    exit 1
fi

echo "Listing weights:"
curl -X GET http://$HOST/api/weights

# echo "Creating weight:"
# curl -X POST $HOST/api/weights -H "Content-Type: application/json" -d '{"weight_kg": 66.5, "measured_at": "2022-11-04T12:34:56" }'

# echo "Listing weights:"
# curl -X GET $HOST/api/weights
