#!/usr/bin/env bash

set -eo pipefail

date

rustup component add rustfmt

echo 'running rustfmt'

cargo fmt

echo 'success!'

echo 'running tests'

./scripts/test.sh
