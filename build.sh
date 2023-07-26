#!/bin/bash

set -e

cargo install wasm-pack
wasm-pack build --target nodejs --release --scope qxip
