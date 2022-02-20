#!/usr/bin/env bash

set -euo pipefail

# build for CI
cargo build --verbose

# simulator
cargo test -- --nocapture

# unigen
( cd crates/unigen && exec cargo test -- --nocapture )
