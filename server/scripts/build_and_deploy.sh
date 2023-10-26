# On my development machine, I ran these commands:
eval $(ssh-agent)
ssh-add .ssh/keys/johnshaughnessy@github.com
ssh track

# On my remote (ubuntu) machine, I ran these commands:
git clone git@github.com:johnshaughnessy/track.git
cd track
git clone git@github.com:johnshaughnessy/track-db.git db
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
sudo apt update
sudo apt install libc
sudo apt install build-essential
sudo apt install libsqlite3-dev
cargo build
sudo APP_ENV=prod ./target/debug/server
