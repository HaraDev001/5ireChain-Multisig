#!/usr/bin/env bash

set -e
./target/release/node-5ire --chain ./chains/testnet/5fire.json -d data/validator3 --name validator3 --validator --port 30335 --ws-port 9947 --rpc-port 9935 --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --node-key 0x2b15acbb4f040d38ceeda2901876e9249843b55d3b26c01a321a63ec6d05d1f1
