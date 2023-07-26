#!/bin/bash

curl -sL https://deb.nodesource.com/setup_18.x | bash -
apt-get install -y nodejs

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

npm i
npm run build