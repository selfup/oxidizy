#!/usr/bin/env bash

set -eo pipefail

cargo build --quiet --package unigen

time cargo run --quiet --package unigen -- $1
