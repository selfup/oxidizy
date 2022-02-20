#!/usr/bin/env bash

set -eo pipefail

diff="rust updates"
no_diff="no rust updates"
gs_check=$(git status -s)
rust_checks=$(
    (echo $gs_check | grep -e '.rs' -e 'Cargo') && echo $diff || echo $no_diff
)

date

if [[ $rust_checks == $no_diff ]]
then
    echo 'no need to run rust CI scripts..'
    echo 'exiting succesfully..'
    exit 0
fi

if [[ $rust_checks == $diff ]]
then
    rustup component add rustfmt

    echo 'running rustfmt'

    cargo fmt

    echo 'success!'

    echo 'running tests'

    ./scripts/test.sh
fi
