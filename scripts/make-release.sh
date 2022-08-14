#!/bin/bash

cd "$(dirname "$0")/.."

rm dist/sqlite-migrator 2> /dev/null

cargo build --target=x86_64-unknown-linux-gnu --release
cp target/x86_64-unknown-linux-gnu/release/sqlite-migrator dist
