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

echo 'running dev.generate'

./scripts/dev.generate.sh
./scripts/dev.generate.sh 20

echo 'dev.generate success!'

echo 'running generate arg'

./scripts/generate.sh
./scripts/generate.sh 20

echo 'generate success!'
