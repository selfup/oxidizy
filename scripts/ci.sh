#!/usr/bin/env bash

set -eo pipefail

date

git status -s | grep -q -e '.rs' -e 'Cargo.'

if [[ $? != 0 ]]
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
