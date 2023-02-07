Get-Date

rustup component add rustfmt

Write-Output 'running rustfmt'

cargo fmt --all -- --check

Write-Output 'format success!'

Write-Output 'running tests'

.\pwsh\test.ps1

Write-Output 'ci success!'

Write-Output 'running dev.generate'

.\pwsh\dev.generate.ps1
.\pwsh\dev.generate.ps1 20

Write-Output 'generate with arg success!'

Write-Output 'running generate without arg'

.\pwsh\generate.ps1
.\pwsh\generate.ps1 20

Write-Output 'generate without arg success!'
