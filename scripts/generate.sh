#!/usr/bin/env bash

set -eo pipefail

cargo build --release --quiet --package unigen

time cargo run --release --quiet --package unigen -- $1
