#!/usr/bin/env bash

set -e
./target/release/node-5ire --chain ./chains/testnet/5fire.json -d data/validator2 --name validator2 --validator --port 30334 --ws-port 9946 --rpc-port 9934 --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --node-key 0x5e04b3b9a7eeaba22193cddc822aea3d08046b0f9fdcaeeb09b0fb5206529edc
