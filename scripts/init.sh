#!/usr/bin/env bash
# This script is meant to be run on Unix/Linux based systems
set -e

echo "*** Initializing WASM build environment"

if [ -z "$CI_PROJECT_NAME" ] ; then
   rustup update nightly
   rustup update stable
fi
rustup install stable
rustup default stable
rustup target add wasm32-unknown-unknown --toolchain nightly
