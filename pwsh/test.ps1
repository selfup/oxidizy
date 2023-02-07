# build for CI
cargo build --verbose

# simulator
cargo test -- --nocapture

# unigen
Set-Location crates/unigen

cargo test -- --nocapture

Set-Location ../..
