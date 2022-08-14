#!/bin/bash

cd "$(dirname "$0")/.."

cargo build --target=x86_64-unknown-linux-gnu --release
cp target/x86_64-unknown-linux-gnu/release/sqlite-manager dist
