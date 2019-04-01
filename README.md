
# Raspberry Pi SSD1306 Rust Example

Example project that cross-compiles Rust code to drive an Adadruit [PiOLED](https://www.adafruit.com/product/3527).

Uses the [raspberry-pi-cross-compiler](https://github.com/sdt/docker-raspberry-pi-cross-compiler) Docker image.

The Rust version used is pinned in the Dockerfile.

## Build

### Raspberry Pi Zero / 1
```
./cross-compile-rpi-1-0.sh
```

### Raspberry Pi 2 / 3
```
./cross-compile-rpi-2-3.sh
```
