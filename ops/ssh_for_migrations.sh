#!/bin/bash

# Get the name of the script
script_name="$0"

# Get the directory of the currently executing script
dir="$(dirname "$0")"

# Argument: IP address or hostname of the VM
VM_IP=$1

# Validate that VM_IP is not empty
if [[ -z "$VM_IP" ]]; then
    echo "Please provide the VM's IP address or hostname as an argument."
    exit 1
fi

scp ~/.keys/hubs-dev-333333-f9959bfb187a.json john@$VM_IP:/tmp/gcp-credentials.json
scp "$dir/run_migration_runner.sh" john@$VM_IP:/home/john/track/run_migration_runner.sh

ssh john@$VM_IP "chmod +x /home/john/track/run_migration_runner.sh"

echo ""
echo "---------------------------------------------"
echo ""
echo "  Running diesel "
echo ""
echo "---------------------------------------------"
echo ""
echo "# For diesel cli commands, run:"
echo ""
echo "    ~/track/run_migration_runner.sh"
echo ""
echo "# Then, from inside the docker container, run:"
echo ""
echo "    export DATABASE_URL=postgres://postgres:redacted@db:5432/postgres"
echo ""
echo "    diesel migration list"
echo ""
echo "# You will need to set the DATABASE_URL (and postgres password)."
echo ""
echo "-------------------------------------------"
echo ""

# SSH into your target machine
ssh "john@$VM_IP"
