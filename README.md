
# Inertia

## Prepare
WINDOWS: Download binaries of pico-sdk-tools, picotool and openocd
```bash
./prepare-windows.sh
```

Update cargo and rustc
```
rustup update
```

Add targets 
```
rustup target add thumbv8m.main-none-eabihf
rustup target add riscv32imac-unknown-none-elf
```
Install project dependencies
```
cargo update
```

## Build
```
cargo build
```
To build a particular app, e.g. the wifi:
```
cargo build --target wifi
```
See all apps in `src/bin/`

## Flash
The device must be in **BOOTSEL mode**. To do so, hold the pico button when plugging its usb to the computer. The pico will be mounted as a flashdrive. 
Use the `--bin` option to specify the app name to flash
```
cargo run --bin wifi
```

## Troubleshoot

### When flashing
#### No accessible RP-series devices in BOOTSEL mode were found
```
No accessible RP-series devices in BOOTSEL mode were found.
error: process didn't exit successfully: `picotool load -u -v -x -t elf 'target\thumbv8m.main-none-eabihf\debug\wifi'` (exit code: 0xfffffff9)
```
The pico must be in **BOOTSEL mode**. See Flash instructions above