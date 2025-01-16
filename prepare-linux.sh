#!/bin/bash
git clone https://github.com/raspberrypi/pico-sdk-tools pico
pushd pico
./build_linux.sh
popd