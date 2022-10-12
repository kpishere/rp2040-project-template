#!/bin/sh
nc localhost 8765 | defmt-print -e target/thumbv6m-none-eabi/debug/rp2040-project-template
