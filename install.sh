#/bin/sh

cargo build --release
sudo cp target/release/findthat /usr/local/bin/
sudo chmod +x /usr/local/bin/findthat
echo DONE!