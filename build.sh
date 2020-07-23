#!/bin/sh

for bdf in bdf/*.bdf; do
    cargo run "$bdf" png/"$(basename "/${bdf%.bdf}.png")"
done
