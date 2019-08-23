#!/bin/bash
cargo build
cd target/thumbv6m-none-eabi/debug
arm-none-eabi-objcopy -O binary stm32l0x2_blinky stm32l0x2_blinky.bin
cd -
openocd
# bobbin load --bin stm32_blink