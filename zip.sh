#!/bin/bash
rm -r zip ants.7z || true
mkdir -p zip
cp -r assets zip
cp target/release/bevy-ants zip
7z a ants.7z $(find zip)
du -h ants.7z
