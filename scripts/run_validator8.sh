#!/usr/bin/env bash

set -e
./target/release/node-5ire --chain ./chains/testnet/5fire.json -d data/validator8 --name validator8 --validator --port 30340 --ws-port 9952 --rpc-port 9940 --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --node-key 0x351cd98cfac65e56a4fb98b1b90b7b182fdb89f20f0a673c5ce47259843fe9da
