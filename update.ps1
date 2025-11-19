<#
    .SYNOPSIS
        Creates an item filter for Path of Exile.
    
    .DESCRIPTION
        Runs the underlying filter-maker rust program, which uses a TOML configuration file
        to write a new item filter for Path of Exile 1 or 2. A;so makes sure rust is installed
        before attempting to run the filter-maker program.
    
    .PARAMETER Filter
        A relative path to the TOML configuration file.

    .PARAMETER Destination
        An optional absolute path for the destination of the generated filter. If it is not
        provided, the filter-maker application will do it's best to find where Path of Exile
        is installed and copy the filter there for you.

    .EXAMPLE
        PS> ./update.ps1 -Filter ./config/filter.toml

        Looks for a TOML configuration file named "filter.toml" in the config/ directory, and
        copies the new item filter to the default Path of Exile installation directory.

        PS> ./update.ps1 -Filter ./config/filter.poe1.toml -Destination "C:/Path of Exile"

        Looks for a TOML configuration file named "filter.poe1.toml" in the config/ directory,
        and copies the new item filter to the "C:/Path of Exile" directory.
#>

param(
    [Parameter(Mandatory=$false)]
    [switch] $poe2,
    [string] $Filter,
    [string] $Destination
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

function Write-NewFilter([string] $FilterPath, [string] $DestinationPath) {
    Write-Host "Generating new filter..." -ForegroundColor DarkGray
    # if ($FilterPath -eq "poe2") {
    #     $FilterPath = "config/filter.poe2.toml"
    #     $DestinationPath = (Get-ChildItem -Path ([Environment]::GetFolderPath([Environment+SpecialFolder]::MyDocuments)) -Directory -Recurse -Filter "Path of Exile 2" -ErrorAction SilentlyContinue).FullName
    # } elseif ([string]::IsNullOrWhiteSpace($Filterpath)) {
    #     $FilterPath = "config/filter.poe1.toml"
    #     $DestinationPath = (Get-ChildItem -Path ([Environment]::GetFolderPath([Environment+SpecialFolder]::MyDocuments)) -Directory -Recurse -Filter "Path of Exile" -ErrorAction SilentlyContinue).FullName
    if (-not $poe2) {
        $FilterPath = "config/filter.poe1.toml"
        $DestinationPath = (Get-ChildItem -Path ([Environment]::GetFolderPath([Environment+SpecialFolder]::MyDocuments)) -Directory -Recurse -Filter "Path of Exile" -ErrorAction SilentlyContinue).FullName        
    }
    else {
        $FilterPath = "config/filter.poe2.toml"
        $DestinationPath = (Get-ChildItem -Path ([Environment]::GetFolderPath([Environment+SpecialFolder]::MyDocuments)) -Directory -Recurse -Filter "Path of Exile 2" -ErrorAction SilentlyContinue).FullName     
    }
    
    if ([string]::IsNullOrWhiteSpace($DestinationPath)) {
        cargo run -- $FilterPath
    }else{
        cargo run -- $FilterPath $DestinationPath
    return $LASTEXITCODE -eq 0
    }
}

function Update-Filter([string] $FilterPath, [string] $DestinationPath) {
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
    if ($false -eq (Write-NewFilter -FilterPath $FilterPath -DestinationPath $DestinationPath)) {
        Write-Host "There was an issue generating the new filter." -ForegroundColor DarkRed
        exit 0
    }

    Write-Host "✓ All done!"
}

Update-Filter -FilterPath $Filter -DestinationPath $Destination