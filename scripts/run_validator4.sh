#!/usr/bin/env bash

set -e
./target/release/node-5ire --chain ./chains/testnet/5fire.json -d data/validator4 --name validator4 --validator --port 30336 --ws-port 9948 --rpc-port 9936 --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --node-key 0x2d9b93502b85c543b7a8f2b3dd5b6b027841a8c47dc9ff133a99a864bba57878
