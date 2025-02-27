#!/bin/sh

set -e

cd "$(dirname "$0")"

cd ..

cargo run assets/test.json output.png
