#!/usr/bin/env bash

set -e
./target/release/node-5ire --chain ./chains/testnet/5fire.json -d data/validator5 --name validator5 --validator --port 30337 --ws-port 9949 --rpc-port 9937 --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --node-key 0x2b7424678eb0c5638f981418334418bd44bc6a7736b40da3ab53140e4062a64d
