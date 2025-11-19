<#
    .SYNOPSIS
        Creates an item filter for Path of Exile.
    
    .DESCRIPTION
        Runs the underlying filter-maker rust program, which uses a TOML configuration file
        to write a new item filter for Path of Exile 1 or 2. Also makes sure rust is installed
        before attempting to run the filter-maker program.

    .PARAMETER Poe2
        An optional switch that will build the new item filter and then copy it to the Path
        of Exile 2 installation folder instead.
    
    .PARAMETER Filter
        An optional relative path to a custom TOML configuration file.
#>

param(
    [switch] $Poe2,
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

function Get-FilterPath([string] $FilterPath) {
    # allow a user to define their own filter path on the script
    if (![string]::IsNullOrWhiteSpace($FilterPath)) {
        return $FilterPath
    }

    # otherwise infer the filter path based on the -Poe2 switch
    return $Poe2 ? "config/filter.poe2.toml" : "config/filter.poe1.toml"
}

function Get-DestinationPath {
    $poeInstallDir = $Poe2 ? "Path of Exile 2" : "Path of Exile"
    return (Get-ChildItem -Path ([Environment]::GetFolderPath([Environment+SpecialFolder]::MyDocuments)) -Directory -Recurse -Filter $poeInstallDir -ErrorAction SilentlyContinue).FullName
}

function Write-NewFilter([string] $FilterPath) {
    Write-Host "Generating new filter..." -ForegroundColor DarkGray
    cargo run -- (Get-FilterPath -FilterPath $FilterPath) (Get-DestinationPath)
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