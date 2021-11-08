#!/bin/bash

curl https://sh.rustup.rs -sSf | sh

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

wasm-pack build

cd web

npm install

npm run build