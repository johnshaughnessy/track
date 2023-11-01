# Copy and paste these commands from inside the target VM to start running migrations!

# Assuming that your GCP service account credentials are stored locally in a file named "gcp-credentials.json"
# You should upload this file securely to your VM beforehand
if [[ ! -f "/tmp/gcp-credentials.json" ]]; then
    echo "Please upload your gcp-credentials.json file to the home directory of this VM."
    exit 1
fi

# Authenticate GCP
gcloud auth activate-service-account --key-file=/tmp/gcp-credentials.json

rm /tmp/gcp-credentials.json

# Configure Docker to use Google Artifact Registry
gcloud auth configure-docker us-central1-docker.pkg.dev

# Run the Docker container
docker run --rm -it -v ~/track:/app --network docker_default us-central1-docker.pkg.dev/hubs-dev-333333/ocho-osai/johnshaughnessy/track/track-base-builder /bin/bash
