#Requires -Version 5
[CmdletBinding()]
param(
    [Parameter(Mandatory = $false, Position = 0, ValueFromRemainingArguments = $true)]
    [object[]] $Arguments
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Show-Usage {
    Write-Host -Object @"
Usage: bootstrap [-h] [--force] [--stdout] [SCRIPT_PATH]

  SCRIPT_PATH  path to wrapper script
  --force      force overwrite of wrapper script if it already exists
  --stdout     write wrapper script to stdout

Visit https://github.com/rcook/isopy for more information
"@
}

function New-TemporaryDirectory {
    $tempDir = [System.IO.Path]::GetTempPath()
    [string] $name = [System.Guid]::NewGuid()
    New-Item -ItemType Directory -Path (Join-Path -Path $tempDir -ChildPath $name)
}


$argv = @() + $Arguments
if (($argv.Length -eq 1) -and ($null -eq $argv[0])) {
    Show-Usage
    exit 1
}

if ($argv -in @('-h', '--help')) {
    Show-Usage
    exit 0
}

$scriptPath = $null
$force = $null
$stdout = $null
while ($null -ne $argv) {
    [string] $arg, [string[]] $argv = $argv
    if ($arg.Length -eq 0) {break}

    if ($arg -in @('-f', '--force')) {
        if ($null -ne $force) {
            Write-Host -Object "--force already specified`n"
            Show-Usage
            exit 1
        }
        $force = $true
        continue
    }

    if ($arg -in @('-o', '--stdout')) {
        if ($null -ne $stdout) {
            Write-Host -Object "--stdout already specified`n"
            Show-Usage
            exit 1
        }
        $stdout = $true
        continue
    }

    if ($arg.StartsWith('-')) {
        Write-Host -Object "Invalid option $arg`n"
        Show-Usage
        exit 1
    }

    if ($null -ne $scriptPath) {
        Write-Host -Object "Script path already specified`n"
        Show-Usage
        exit 1
    }

    [string] $scriptPath = Resolve-Path -Path $arg -ErrorAction SilentlyContinue -ErrorVariable _frperror
    if (-not $scriptPath) {
        $scriptPath = $_frperror[0].TargetObject
    }
}

if ((-not $force) -and ($null -ne $scriptPath) -and (Test-Path -Path $scriptPath)) {
    Write-Host -Object "Script $scriptPath already exists; specify --force to overwrite it"
    exit 1
}

$isopyBranch = 'main'
$isopyUrl = "https://github.com/rcook/isopy/archive/refs/heads/$isopyBranch.zip"
$isopyDir = Join-Path -Path $env:USERPROFILE -ChildPath .isopy
$isopySrcDir = Join-Path -Path $isopyDir -ChildPath src
$isopyAssetsDir = Join-Path -Path $isopyDir -ChildPath assets
$isopyBranchDir = Join-Path -Path $isopySrcDir -ChildPath $isopyBranch
$isopyEnvDir = Join-Path -Path $isopyDir -ChildPath envs\isopy
$isopyEnvPath = Join-Path -Path $isopyEnvDir -ChildPath env.yaml
$isopyEnvPythonDir = Join-Path -Path $isopyEnvDir -ChildPath python
$isopyEnvScriptsDir = Join-Path -Path $isopyEnvPythonDir -ChildPath Scripts

$tempDir = New-TemporaryDirectory
if (-not (Test-Path -Path $isopyBranchDir)) {
    try {
        $tempZipDir = Join-Path -Path $tempDir -ChildPath isopy-$isopyBranch
        $tempZipPath = Join-Path -Path $tempDir -ChildPath isopy-$isopyBranch.zip
        #Invoke-WebRequest -Uri $isopyUrl -OutFile $tempZipPath
        Copy-Item -Path (Join-Path -Path $env:USERPROFILE -ChildPath Desktop/main.zip) -Destination $tempZipPath
        Expand-Archive -Path $tempZipPath -DestinationPath $tempDir
        New-Item -ItemType Directory -Path $isopySrcDir -Force | Out-Null
        $temp = Join-Path -Path $tempDir -ChildPath $isopyBranch
        Rename-Item -Path $tempZipDir -NewName $temp
        Move-Item -Path $temp -Destination $isopySrcDir
    }
    finally {
        if ($null -ne $tempDir) {
            Remove-Item -Recurse -Force -Path $tempDir
        }
    }
}

$yamlPath = Join-Path -Path $isopyBranchDir -ChildPath .isopy.yaml
$c = (Get-Content -Path $yamlPath) -join ' '
$m = [regex]::Match($c, "python_version: '?([^\s']+)'?")
if (-not $m.Success) {throw 'Regex match failed'}
$pythonVersion = $m.Groups[1].Value
$m = [regex]::Match($c, "tag: '?([^\s']+)'?")
if (-not $m.Success) {throw 'Regex match failed'}
$tag = $m.Groups[1].Value
$pythonBaseName = "cpython-$pythonVersion+$tag"
$pythonFileName = "$pythonBaseName-x86_64-pc-windows-msvc-shared-install_only.tar.gz"

$pythonUrl = "https://github.com/indygreg/python-build-standalone/releases/download/$tag/$pythonFileName"
$pythonPath= Join-Path -Path $isopyAssetsDir -ChildPath $pythonFileName

New-Item -ItemType Directory -Path $isopyAssetsDir -Force | Out-Null
New-Item -ItemType Directory -Path $isopyEnvPythonDir -Force | Out-Null

if (-not (Test-Path -Path $pythonPath)) {
    Invoke-WebRequest -Uri $pythonUrl -OutFile $pythonPath
    $hash = (Get-FileHash -Path $pythonPath -Algorithm SHA256).Hash
    $expectedHash = Get-Content -Path (Join-Path -Path $isopyBranchDir -ChildPath sha256sums\$pythonFileName.sha256)
    if ($hash -ne $expectedHash) {
        Remove-Item -Recurse -Force -Path $pythonPath
        Write-Host -Object "Checksum failed on $pythonPath"
        exit 1
    }
}

if (-not (Test-Path -Path $isopyEnvPythonDir\python.exe)) {
    $d = New-TemporaryDirectory
    try {
        & tar xf $pythonPath -C $d.FullName
        $sourceDir = Join-Path -Path $d -ChildPath python
        Move-Item -Path $sourceDir\* -Destination $isopyEnvPythonDir
    }
    finally {
        if ($null -ne $d) {
            Remove-Item -Recurse -Force -Path $d
        }
    }

    try {
        $tempPath = $env:PATH
        $env:Path = $isopyEnvPythonDir + ';' + $isopyEnvScriptsDir + ';' + $env:Path
        & python -m pip install --upgrade pip
        & python -m pip install -r (Join-Path -Path $isopyBranchDir -ChildPath requirements.txt)
    }
    finally {
        $env:Path = $tempPath
    }
}

Set-Content -Path $isopyEnvPath -Value (@"
name: isopy
python_dir: python
python_version: $pythonVersion
tag: '$tag'
"@)


$wrapper = @"
#Requires -Version 5
[CmdletBinding()]
param(
    [Parameter(Mandatory = `$false, Position = 0, ValueFromRemainingArguments = `$true)]
    [object[]] `$Arguments
)

Set-StrictMode -Version Latest
`$ErrorActionPreference = 'Stop'
try {
    `$tempPath = `$env:PATH
    `$tempPythonPath = `$env:PYTHONPATH
    `$env:Path = '$isopyEnvPythonDir' + ';' + ``
        '$isopyEnvScriptsDir' + ';' + ``
        `$env:Path
    `$env:PYTHONPATH = '$isopyBranchDir'
    & python.exe "$isopyBranchDir\isopy_bin\main.py" `$Arguments
}
finally {
    `$env:PYTHONPATH = `$tempPythonPath
    `$env:Path = `$tempPath
}
"@

if ($stdout) {
    Write-Host -Object @"
--------------------------------------------------
Copy and paste this into a PowerShell script on PATH
--------------------------------------------------
"@
    Write-Host -Object $wrapper
    Write-Host -Object @"
--------------------------------------------------

--------------------------------------------------
Or stick this at the top of your PowerShell profile file:
--------------------------------------------------
`$env:Path = '$isopyEnvPythonDir' + ';' + ``
    '$isopyEnvScriptsDir' + ';' + ``
    `$env:Path
--------------------------------------------------
"@
} else {
    Set-Content -Path $scriptPath -Value $wrapper
    Write-Host -Object "Wrapper script generated at $scriptPath; please make sure this file is on PATH"
}
