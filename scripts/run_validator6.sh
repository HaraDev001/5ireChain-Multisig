#!/usr/bin/env bash

set -e
./target/release/node-5ire --chain ./chains/testnet/5fire.json -d data/validator6 --name validator6 --validator --port 30338 --ws-port 9950 --rpc-port 9938 --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --node-key 0x1f22430216727733ab383001d3c7ad624f073af0fd7bb515e808b5b327f6e2ec
