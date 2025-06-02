#!/bin/bash

cargo build --bin spi_display --target thumbv6m-none-eabi
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/spi_display
