Get-Date

rustup component add rustfmt

Write-Output 'running rustfmt'

cargo fmt --all -- --check

Write-Output 'rustfmt success!'

Write-Output 'running tests'

.\pwsh\test.ps1

Write-Output 'tests success!'

Write-Output 'running dev.generate'

.\pwsh\dev.generate.ps1
.\pwsh\dev.generate.ps1 20

Write-Output 'dev.generate success!'

Write-Output 'running generate'

.\pwsh\generate.ps1
.\pwsh\generate.ps1 20

Write-Output 'generate success!'

Write-Output 'ci success!'
