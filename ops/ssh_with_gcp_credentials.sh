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

cat "$dir/configure_migration_runner.sh"

echo ""

# SSH into your target machine
ssh john@$VM_IP
