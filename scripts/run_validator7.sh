#!/usr/bin/env bash

set -e
./target/release/node-5ire --chain ./chains/testnet/5fire.json -d data/validator7 --name validator7 --validator --port 30339 --ws-port 9951 --rpc-port 9939 --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --node-key 0x7717785eef4b8fb43beb6ee386e869a79a7fbd819c48c071638d70b35718f8da
