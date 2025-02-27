#!/bin/sh

set -e

cd "$(dirname "$0")"

cd ../lib/schema-generator

cargo run
