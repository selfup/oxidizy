Get-Date

rustup component add rustfmt

Write-Output 'running rustfmt'

cargo fmt --all -- --check

Write-Output 'rustfmt success!'

Write-Output 'running tests'

.\pwsh\test.ps1

Write-Output 'tests success!'

Write-Output 'ci success!'
