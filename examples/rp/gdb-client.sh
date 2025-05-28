#!/bin/bash

arm-none-eabi-gdb target/thumbv6m-none-eabi/release/wifi_blinky

# target remote :1337
# break wifi_blinky::__cortex_m_rt_main