#!/bin/sh
set -e
cargo build --release;

sudo mkdir -p /opt/gosh
sudo cp target/release/gosh /opt/gosh/gosh 
sudo cp -r std /opt/gosh/std
