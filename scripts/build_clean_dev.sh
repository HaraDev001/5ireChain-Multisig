#!/usr/bin/env bash

set -e

cargo build --release
./target/release/node-5ire purge-chain --dev -y
./target/release/node-5ire --dev
