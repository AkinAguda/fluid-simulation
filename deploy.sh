#!/bin/bash

echo -e "\e[1;33m BUILDING... Installing ans setting up cargo \e[0m"
curl https://sh.rustup.rs -sSf -y | sh &&
echo -e "\e[1;32m BUILDING... Cargo Installed sucessfully \e[0m"

echo -e "\e[1;33m BUILDING... Installing wasm-pack \e[0m"
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf -y | sh &&
echo -e "\e[1;32m BUILDING... wasm-pack Installed sucessfully \e[0m"

echo -e "\e[1;33m BUILDING... Building webassembly module \e[0m"
wasm-pack build &&
echo -e "\e[1;32m BUILDING... web assembly module built sucessfully \e[0m"

cd web &&

echo -e "\e[1;33m BUILDING... Installing web dependencies \e[0m"
npm install &&
echo -e "\e[1;32m BUILDING... Dependencies installed sucessfully \e[0m"

echo -e "\e[1;33m BUILDING... Building application \e[0m"
npm run build &&
echo -e "\e[1;32m BUILT!!!... Application built sucessfully \e[0m"