function Test-RustInstallation {
    try {
        rustc --version
        return $true
    }
    catch {
        return $false
    }
}

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

function Update-Filter {
    # Make sure rust is installed
    if ($false -eq (Test-RustInstallation)) {
        Write-Host "Please install rust to generate your filter by visiting https://www.rust-lang.org/tools/install." -ForegroundColor White
        exit 0
    }

    # Create a filter config file if one does not exist
    if ($false -eq (Test-Path -Path "config/filter.toml")) {
        Write-Host "No filter config file detected; creating one from the example..." -ForegroundColor DarkGray
        Copy-Item -Path "config/filter.example.toml" -Destination "config/filter.toml"
    }

    # Get the latest code
    if ($false -eq (Get-LatestCode)) {
        Write-Host "There was an issue getting the latest version. New filter will not be generated." -ForegroundColor DarkRed
        exit 0
    }

    # Generate the filter
    if ($false -eq (Write-NewFilter)) {
        Write-Host "There was an issue generating the new filter." -ForegroundColor DarkRed
        exit 0
    }

    Write-Host "✓ All done!"
}

Update-Filter