#!/bin/bash

set -e

apt-get update
apt-get install -yq build-essential curl git

curl -sL https://deb.nodesource.com/setup_18.x | bash -
apt-get update
apt-get install -yq nodejs

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

npm i
npm run build
npm run test
