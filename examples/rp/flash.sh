#!/bin/bash

cargo build --bin wifi_blinky --target thumbv6m-none-eabi
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/wifi_blinky
