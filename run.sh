#!/usr/bin/env bash
#open openocd in other a terminal
#cargo clean
cargo build
#cargo run --example hello
#cargo run --example sysTick
#cargo run --example pac
#cargo run --example panic
#cargo run --example exception
#cargo run --example crash
#cargo run --example first_attempt
cargo run --example ledToggle

#as described in  https://rust-embedded.github.io/book/start/hardware.html at the end