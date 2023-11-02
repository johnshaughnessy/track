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

# Ensure Nix is installed
ssh -T john@$VM_IP <<'ENDSSH'
  if ! command -v nix-env &> /dev/null; then
    echo "Nix not found. Installing Nix..."
    curl -L https://nixos.org/nix/install | sh
    # . $HOME/.nix-profile/etc/profile.d/nix.sh
  else
    echo "Nix is already installed. Skipping installation."
  fi
ENDSSH

# Copy Nix configuration to VM
ssh -T john@$VM_IP "mkdir -p /home/john/track/"
echo "Copying configs..."
scp ./vm-config.nix john@$VM_IP:/home/john/track/
scp ./docker.service john@$VM_IP:/home/john/track/
scp ./.bashrc john@$VM_IP:/home/john/.bashrc

# Apply Nix configuration on VM
ssh -T john@$VM_IP <<'ENDSSH'
  # Update PATH to include Nix binaries
  export PATH=$PATH:/nix/var/nix/profiles/default/bin

  # Make sure Nix is properly set up for this session
  # source ~/.nix-profile/etc/profile.d/nix.sh

  # Install the packages
  nix-env -i -f /home/john/track/vm-config.nix

  sudo cp /home/john/track/docker.service /etc/systemd/system/docker.service

  # Check if 'docker' group exists
  getent group docker >/dev/null 2>&1 || sudo groupadd docker

  # Check if user is already in 'docker' group
  if ! id -nG "$USER" | grep -qw "docker"; then
    sudo usermod -aG docker $USER
  fi

  sudo systemctl daemon-reload
  sudo systemctl enable docker
  sudo systemctl restart docker
  echo "Docker is $(sudo systemctl is-active docker)."
ENDSSH
