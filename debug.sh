#!/usr/bin/env bash

# do each step manually

#gdb -q target/thumbv7em-none-eabihf/debug/examples/hello
#target remote :3333
#load
#monitor arm semihosting enable
#break main


#use openocd.gdb

gdb -x openocd.gdb -q target/thumbv7em-none-eabihf/debug/examples/hello