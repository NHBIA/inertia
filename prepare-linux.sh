#!/bin/bash
rustup target add thumbv8m.main-none-eabihf
rustup target add riscv32imac-unknown-none-elf

git clone https://github.com/raspberrypi/pico-sdk-tools pico
pushd pico
./build_linux.sh
popd