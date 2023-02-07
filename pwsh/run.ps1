param(
    [Parameter()]
    [String]$size = ""
)

if ($size -eq "") {
    cargo run -q
}
else {
    cargo run -q $size
}
