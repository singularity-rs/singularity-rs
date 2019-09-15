#!/bin/bash

# sudo apt-get install libasound2-dev pkg-config libssl-dev cmake libfreetype6-dev libexpat1-dev libxcb-composite0-dev libx11-xcb-dev

# rustup component add clippy

# first updating rust
echo Updating Rust version
rustup update

echo Testing with default version
rustfmt --check src/**.rs
cargo $1 check --all --no-default-features --features empty
cargo $1 clippy -- -D warnings
cargo $1 build --all --no-default-features --features empty
cargo $1 test  --all --no-default-features --features empty

#
# echo Testing stable Rust
# cargo +stable check --all --no-default-features --features empty
# cargo +stable clippy -- -D warnings
# cargo +stable build --all --no-default-features --features empty
# cargo +stable test  --all --no-default-features --features empty
#
#
#
# # checking with beta
# rustup install beta
#
# echo Testing Rust beta
# cargo +beta check --all --no-default-features --features empty
# cargo +beta clippy -- -D warnings
# cargo +beta build --all --no-default-features --features empty
# cargo +beta test  --all --no-default-features --features empty
#
#
#
# # checking with nightly
# rustup install nightly
#
# echo Testing Rust nightly
# cargo +nightly check --all --no-default-features --features empty
# cargo +nightly clippy -- -D warnings
# cargo +nightly build --all --no-default-features --features empty
# cargo +nightly test  --all --no-default-features --features empty







