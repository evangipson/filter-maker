<#
    .SYNOPSIS
        Creates an item filter for Path of Exile.
    
    .DESCRIPTION
        Runs the underlying filter-maker rust program, which uses a TOML configuration file
        to write a new item filter for Path of Exile 1 or 2. A;so makes sure rust is installed
        before attempting to run the filter-maker program.
    
    .PARAMETER Filter
        A relative path to the TOML configuration file.

    .EXAMPLE
        PS> ./update.ps1 -Filter ./config/filter.toml

        Looks for a TOML configuration file named "filter.toml" in the config/ directory, and
        uses it to create a new item filter.
#>

param(
    [Parameter(Mandatory=$true)]
    [string] $Filter
)

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

function Write-NewFilter([string] $FilterPath) {
    Write-Host "Generating new filter..." -ForegroundColor DarkGray
    cargo run -- $FilterPath
    return $LASTEXITCODE -eq 0
}

function Update-Filter([string] $FilterPath) {
    # Make sure rust is installed
    if ($false -eq (Test-RustInstallation)) {
        Write-Host "Please install rust to generate your filter by visiting https://www.rust-lang.org/tools/install." -ForegroundColor White
        exit 0
    }

    # Get the latest code
    if ($false -eq (Get-LatestCode)) {
        Write-Host "There was an issue getting the latest version. New filter will not be generated." -ForegroundColor DarkRed
        exit 0
    }

    # Generate the filter
    if ($false -eq (Write-NewFilter -FilterPath $FilterPath)) {
        Write-Host "There was an issue generating the new filter." -ForegroundColor DarkRed
        exit 0
    }

    Write-Host "✓ All done!"
}

Update-Filter -FilterPath $Filter