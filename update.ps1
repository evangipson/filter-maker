function Get-LatestCode {
    Write-Host "Getting latest version of filter-maker..." -ForegroundColor DarkGray
    git pull
    return $LASTEXITCODE -eq 0
}

function Write-NewFilter {
    Write-Host "Generating new filter..." -ForegroundColor DarkGray
    cargo run
    return $LASTEXITCODE -eq 0
}

if ($false -eq (Get-LatestCode)) {
    Write-Host "There was an issue getting the latest version. New filter will not be generated." -ForegroundColor DarkRed
    exit 0
}

if ($false -eq (Write-NewFilter)) {
    Write-Host "There was an issue generating the new filter." -ForegroundColor DarkRed
    exit 0
}

Write-Host "✓ All done!"