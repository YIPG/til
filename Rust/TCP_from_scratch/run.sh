#!/usr/local/bin/zsh
cargo b --release
sudo setcap cap_net_admin=eip ./target/release/TCP_from_scratch
./target/release/TCP_from_scratch
