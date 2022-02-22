
#!/usr/bin/env bash

set -e

echo "It will take a long time. Project will be built twice..."

cargo build --release

./target/release/node-5ire build-spec --disable-default-bootnode --chain staging > ./chains/fire/customSpec5ire.json
./target/release/node-5ire build-spec --chain=./chains/fire/customSpec5ire.json --raw --disable-default-bootnode > ./chains/fire/5fire.json
