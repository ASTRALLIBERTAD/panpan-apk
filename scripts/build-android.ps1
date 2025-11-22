$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$root = Resolve-Path "$scriptDir\.."

if (-not (Get-Command panpan-apk -ErrorAction SilentlyContinue)) {
    Write-Host "panpan-apk not found — attempting to build locally..."
    Push-Location "$root\tools\panpan-apk"
    cargo build --release
    Pop-Location
    $env:PATH = "$root\tools\panpan-apk\target\release;" + $env:PATH
}

Write-Host "Running panpan-apk for example_crate..."
panpan-apk --crate-path "$root\example_crate" --android-template "$root\android" --release --install
