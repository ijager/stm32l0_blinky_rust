STM32L0 blinky
===========

Blinky project for b-l072z-lrwan1 board with the stm32l072 cortex-m0+ microcontroller.


## Flash using openocd


build project `Cargo build`

Create binary: `arm-none-eabi-objcopy -O binary stm32_blink stm32_blink.bin`

run openocd in root of project (it will use openocd.cfg)

```
openocd
```

Open telnet session: `telnet localhost 4444`

Issue the following commands

````
init
reset init
halt
flash write_image erase ./target/thumbv6m-none-eabi/debug/stm32_blink.bin 0x08000000
shutdown
````

## Flash using Bobbin


build project `Cargo build`

Create binary: `arm-none-eabi-objcopy -O binary stm32_blink stm32_blink.bin`

```
bobbin load stm32_blink
```

Note that bobbin uses openocd internally.


## Acknowledgments

This project is based on the [stm32_blink](https://gitlab.com/jounathaen/stm32_blink) by Joathan Klinmt