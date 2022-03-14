#!/usr/bin/env bash

set -eo pipefail

cargo build --release --quiet --package unigen

if [[ $1 == '' ]]
then
    echo 'Please pass a number to the script like so: ./scripts/generate.sh 30'
    echo '$1 arg not given - no size for generator..'
    echo 'Aborting!'
    exit 1
fi

time cargo run --release --quiet --package unigen -- $1
