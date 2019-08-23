STM32L0 blinky
===========

This is the cargo project with the source code for the blogpost: https://jonathanklimt.de/electrics/programming/rust-STM32F103-blink/


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