#!/bin/bash
#
# ./setup_vm.sh gcp1
#

# Argument: IP address or hostname of the VM
VM_IP=$1

if [ -z "$VM_IP" ]; then
    echo "Please provide the VM IP address or hostname as an argument."
    exit 1
fi

ssh -T john@$VM_IP <<'ENDSSH'
  # Disable message of the day
  sudo chmod -x /etc/update-motd.d/*
ENDSSH

# Ensure Nix is installed on the Ubuntu VM
ssh -T john@$VM_IP <<'ENDSSH'
  if ! command -v nix-env &> /dev/null; then
    echo "Nix not found. Installing Nix..."
    curl -L https://nixos.org/nix/install | sh
    # . $HOME/.nix-profile/etc/profile.d/nix.sh
  else
    echo "Nix is already installed."
  fi
ENDSSH

# Copy Nix configuration to VM
ssh -T john@$VM_IP "mkdir -p /home/john/track/"
scp ./vm-config.nix john@$VM_IP:/home/john/track/

# Apply Nix configuration on VM
ssh -T john@$VM_IP <<'ENDSSH'
  # Update PATH to include Nix binaries
  export PATH=$PATH:/nix/var/nix/profiles/default/bin

  # Make sure Nix is properly set up for this session
  # source ~/.nix-profile/etc/profile.d/nix.sh

  # Install the packages
  nix-env -i -f /home/john/track/vm-config.nix
ENDSSH
