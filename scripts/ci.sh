#!/usr/bin/env bash

set -eou pipefail

date

rustup component add rustfmt

echo 'running rustfmt'

cargo fmt --all -- --check

echo 'format success!'

echo 'running tests'

./scripts/test.sh

echo 'ci success!'

sleep 1s

echo 'running generate'

./scripts/generate.sh 50

echo 'generate success!'
