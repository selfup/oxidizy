#!/usr/bin/env bash

set -eo pipefail

date

status=$((git status -s | grep -q -e '.rs' -e 'Cargo.') && echo 1 || echo 0)

if [[ $status != 0 ]]
then
    echo 'no need to run rust CI scripts..'
    echo 'exiting succesfully..'
    exit 0
else
    rustup component add rustfmt

    echo 'running rustfmt'

    cargo fmt --all -- --check

    echo 'success!'

    echo 'running tests'

    ./scripts/test.sh

    exit 0
fi
