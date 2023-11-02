#!/usr/bin/env sh
#
# List backups with:
#
#   gsutil ls gs://ocho-osai/track/pg_data/
#
# Download a backup with:
#
#   gsutil cp gs://ocho-osai/track/pg_data/pg_backup_2023-11-02T14-51-44.sql.gz ./backups/
#
# Run this script with
#
#   ./inspect_backup.sh gs://ocho-osai/track/pg_data/pg_backup_2023-11-02T14-51-44.sql.gz
#
# From there, you can run psql commands to inspect the data.
#

BACKUP_FILE=$1
POSTGRES_PASSWORD=$2

if [ -z "$BACKUP_FILE" ] || [ -z "$POSTGRES_PASSWORD" ]; then
    echo "Warning: Both BACKUP_FILE and POSTGRES_PASSWORD need to be provided."
    exit 1
fi

DECOMPRESSED_FILE="${BACKUP_FILE%.*}" # Removes .gz extension
VOLUME_NAME="postgres-data"

# Check if the backup file exists and is a gzip file
if [ -f "$BACKUP_FILE" ] && [[ $BACKUP_FILE == *.gz ]]; then
    echo "Decompressing backup file..."
    gunzip -k "$BACKUP_FILE"
else
    echo "Backup file does not exist or is not a gzip file."
    exit 1
fi

docker container stop "temp-postgres"
docker container rm "temp-postgres"
docker volume rm "$VOLUME_NAME"

# Create a Docker volume (if it doesn't already exist)
echo "Creating Docker volume..."
docker volume create "$VOLUME_NAME"

echo "$DECOMPRESSED_FILE"

# Run the PostgreSQL container
echo "Starting PostgreSQL container..."
docker run --name temp-postgres \
    -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
    -v "$PWD/$DECOMPRESSED_FILE:/docker-entrypoint-initdb.d/dump.sql" \
    -v "$VOLUME_NAME:/var/lib/postgresql/data" \
    -d trackdb

# Check if container started successfully
if [ $? -ne 0 ]; then
    echo "Failed to start the PostgreSQL container."
    exit 1
fi

echo "PostgreSQL container started successfully."

docker exec -it temp-postgres bash -c "sleep 1; psql -U postgres -d trackdb"
