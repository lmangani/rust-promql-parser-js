#!/bin/bash

set -e

apt update
apt-get -yqq install binaryen

cargo install wasm-pack
wasm-pack build --target nodejs --release --scope qxip
