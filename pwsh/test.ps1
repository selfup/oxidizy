# build for CI
cargo build --verbose

# simulator
cargo test -- --nocapture
