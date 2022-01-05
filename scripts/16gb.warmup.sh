#!/usr/bin/env bash

set -eo pipefail

cd crates/unigen

cargo build -q --release

echo '
---
Warming up!
---
'


time cargo run -q --release 260

sleep 3s

echo '
---
16GB run...
---
'

time cargo run -q --release 360

cd ../../
