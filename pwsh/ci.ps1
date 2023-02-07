Get-Date

rustup component add rustfmt

Write-Output 'running rustfmt'

cargo fmt --all -- --check

Write-Output 'format success!'

Write-Output 'running tests'

./scripts/test.ps1

Write-Output 'ci success!'

Write-Output 'running generate with arg'

./scripts/generate.ps1 -size 50

Write-Output 'generate with arg success!'

Write-Output 'running generate without arg'

./scripts/generate.sh

Write-Output 'generate without arg success!'
