#!/bin/bash

# Function to download file without curl but with uv
download_with_python() {
    local URL="$1"
    local OUTPUT="$2"
    uv run python -c "import urllib.request; urllib.request.urlretrieve('$URL', '$OUTPUT')"
}

# Define the URLs for the prebuilt pico-sdk and picotools
PICO_SDK_URL="https://github.com/raspberrypi/pico-sdk-tools/releases/download/v2.1.0-0/pico-sdk-tools-2.1.0-x64-win.zip"
PICO_TOOLS_URL="https://github.com/raspberrypi/pico-sdk-tools/releases/download/v2.1.0-0/picotool-2.1.0-x64-win.zip"
PICO_OPENOCD_URL="https://github.com/raspberrypi/pico-sdk-tools/releases/download/v2.1.0-0/openocd-0.12.0+dev-x64-win.zip"

# Define the special directory and the directories to extract the files
SPECIAL_DIR="sdk_and_tools"
PICO_SDK_DIR="$SPECIAL_DIR/pico-sdk-tools"
PICO_TOOLS_DIR="$SPECIAL_DIR/picotool"
PICO_OPENOCD_DIR="$SPECIAL_DIR/openocd"



# Create directories if they don't exist
mkdir -p $PICO_SDK_DIR
mkdir -p $PICO_TOOLS_DIR
mkdir -p $PICO_OPENOCD_DIR

# Download and extract openocd
#curl -L $PICO_OPENOCD_URL -o openocd.zip
download_with_python $PICO_OPENOCD_URL openocd.zip
#unzip openocd.zip -d $PICO_OPENOCD_DIR
7z x openocd.zip -o$PICO_OPENOCD_DIR
rm openocd.zip

# Download and extract pico-sdk
#curl -L $PICO_SDK_URL -o pico-sdk.zip
download_with_python $PICO_SDK_URL pico-sdk.zip
#unzip pico-sdk.zip -d $PICO_SDK_DIR
7z x pico-sdk.zip -o$PICO_SDK_DIR

rm pico-sdk.zip

# Download and extract picotools
#curl -L $PICO_TOOLS_URL -o picotool.zip
download_with_python $PICO_TOOLS_URL picotool.zip
#unzip picotool.zip -d $PICO_TOOLS_DIR
7z x picotool.zip -o$PICO_TOOLS_DIR
rm picotool.zip

echo "Pico SDK and Picotools have been downloaded and extracted to $SPECIAL_DIR."

#generate .env file with to path for each directory

echo "PICO_SDK_PATH=$PWD/$PICO_SDK_DIR/pioasm" > .env
echo "PICO_TOOL_PATH=$PWD/$PICO_TOOLS_DIR/picotool" >> .env