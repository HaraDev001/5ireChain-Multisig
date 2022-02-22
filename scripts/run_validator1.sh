#!/usr/bin/env bash

set -e
./target/release/node-5ire --chain ./chains/testnet/5fire.json -d data/validator1 --name validator1 --in-peers 256 --validator --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --node-key 0x5e1061dbb97bf47b4012b52dffdec98b67abb7f2dc664fd5c0353542261da130
