#!/usr/bin/env bash

set -euox pipefail

cargo test --release -- --nocapture
