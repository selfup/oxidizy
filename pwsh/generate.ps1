param(
    [Parameter()]
    [String]$size = "200"
)

cargo build --release --quiet --package unigen

$sw = [Diagnostics.Stopwatch]::StartNew()

cargo run --release --quiet --package unigen -- $size

$sw.Stop()
$sw.Elapsed
