#!/usr/bin/env bash
#open openocd in other a terminal
cargo build
#cargo run --example hello
#cargo run --example sysTick
#cargo run --example pac
cargo run --example panic

#as described in  https://rust-embedded.github.io/book/start/hardware.html at the end