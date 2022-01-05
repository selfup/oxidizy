#!/usr/bin/env bash

set -eo pipefail

cd crates/unigen

cargo build -q --release

echo '
---
Warming up!
---
'


time cargo run -q --release 400

sleep 4s

echo '
---
32GB run...
---
'

time cargo run -q --release 480

cd ../../
