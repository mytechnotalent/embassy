#!/bin/bash

probe-rs gdb \
    target/thumbv6m-none-eabi/release/wifi_blinky \
    --chip RP2040
