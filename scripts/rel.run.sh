#!/usr/bin/env bash

set -eox pipefail

if [[ $1 != "" ]]
then
    cargo run -q --release "$1"
else
    cargo run -q --release
fi
