
# Inertia

## Prepare
WINDOWS: Download binaries of pico-sdk-tools, picotool and openocd
```bash
./prepare-windows.sh
```
Then, source the file activate-windows.sh temporarly add the binaries path to your environment
```bash
source activate.sh
```

Update cargo and rustc
```bash
rustup update
```

Add targets 
```bash
rustup target add thumbv8m.main-none-eabihf
rustup target add riscv32imac-unknown-none-elf
```
Install project dependencies
```bash
cargo update
```

## Build
```bash
cargo build
```
To build a particular app, e.g. the wifi:
```bash
cargo build --target wifi
```
See all apps in `src/bin/`

## Flash
The device must be in **BOOTSEL mode**. To do so, hold the pico button when plugging its usb to the computer. The pico will be mounted as a flashdrive. 
Use the `--bin` option to specify the app name to flash
```bash
cargo run --bin wifi
```

## Troubleshoot

### When flashing
#### could not execute process picotool
```bash
error: could not execute process `picotool load -u -v -x -t elf 'target\thumbv8m.main-none-eabihf\debug\wifi'` (never executed)
```
The picotool must be in your path. Make sure picotool is installed and visible in your path. (see above section "Prepare"). 


#### No accessible RP-series devices in BOOTSEL mode were found
```bash
No accessible RP-series devices in BOOTSEL mode were found.
error: process didn't exit successfully: `picotool load -u -v -x -t elf 'target\thumbv8m.main-none-eabihf\debug\wifi'` (exit code: 0xfffffff9)
```
The pico must be in **BOOTSEL mode**. See Flash instructions above