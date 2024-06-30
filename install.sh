#!/bin/bash

echo "Installing futils"

cargo build --release

BIN_DIR="$HOME/.cargo/bin"

mv target/release/futils target/release/fs
mv target/release/fs "$BIN_DIR/"

echo "Successfully installed. try to run fs -h"
