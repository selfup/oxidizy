param(
    [Parameter()]
    [String]$size = ""
)

if ($size -eq "") {
    cargo run -q --release
}
else {
    cargo run -q --release $size
}
