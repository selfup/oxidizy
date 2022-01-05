#!/usr/bin/env bash

set -eo pipefail

cd crates/unigen

cargo build -q --release

echo '
---
Warming up!
---
'

time cargo run -q --release 200

sleep 2s

echo '
---
8GB run...
---
'

time cargo run -q --release 260

cd ../../
