param(
    [Parameter()]
    [String]$size = "30"
)

cargo build --quiet --package unigen

cargo run --quiet --package unigen -- $size
