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

echo 'running generate with arg'

./scripts/generate.sh 50

echo 'generate with arg success!'

echo 'running generate without arg'

./scripts/generate.sh

echo 'generate without arg success!'
