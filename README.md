
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