#Requires -Version 5
[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function New-TemporaryDirectory {
    $tempDir = [System.IO.Path]::GetTempPath()
    [string] $name = [System.Guid]::NewGuid()
    New-Item -ItemType Directory -Path (Join-Path -Path $tempDir -ChildPath $name)
}


$cacheDir = Join-Path -Path $env:USERPROFILE -ChildPath .isopy
$pythonVersion = '3.11.1'
$tag = '20230116'

$assetsDir = Join-Path -Path $cacheDir -ChildPath assets
$isopyEnvDir = Join-Path -Path $cacheDir -ChildPath envs\isopy
$isopyEnvPath = Join-Path -Path $isopyEnvDir -ChildPath env.yaml
$pythonBaseName = "cpython-$pythonVersion+$tag"
$isopyPythonDir = Join-Path -Path $isopyEnvDir -ChildPath python
$isopyScriptsDir = Join-Path -Path $isopyPythonDir -ChildPath Scripts
$binDir = Join-Path -Path $env:USERPROFILE -Child .local\bin

$pythonFileName = "$pythonBaseName-x86_64-pc-windows-msvc-shared-install_only.tar.gz"

$pythonUrl = "https://github.com/indygreg/python-build-standalone/releases/download/$tag/$pythonFileName"
$pythonPath= Join-Path -Path $assetsDir -ChildPath $pythonFileName

New-Item -ItemType Directory -Path $assetsDir -Force | Out-Null
New-Item -ItemType Directory -Path $isopyPythonDir -Force | Out-Null

if (-not (Test-Path -Path $pythonPath)) {
    Invoke-WebRequest -Uri $pythonUrl -OutFile $pythonPath
    $hash = (Get-FileHash -Path $pythonPath -Algorithm SHA256).Hash
    $expectedHash = Get-Content -Path $PSScriptRoot\sha256sums\$pythonFileName.sha256
    if ($hash -ne $expectedHash) {
        Remove-Item -Recurse -Force -Path $pythonPath
        throw "Checksum failed on $pythonPath"
    }
}

if (-not (Test-Path -Path $isopyPythonDir\python.exe)) {
    $d = New-TemporaryDirectory
    try {
        & tar xf $pythonPath -C $d.FullName
        $sourceDir = Join-Path -Path $d -ChildPath python
        Move-Item -Path $sourceDir\* $isopyPythonDir
    }
    finally {
        if ($null -ne $d) {
            Remove-Item -Recurse -Force -Path $d
        }
    }
}

try {
    $tempPath = $env:PATH
    $env:PATH = $isopyPythonDir + ';' + $isopyScriptsDir + ';' + $env:PATH
    & python -m pip install --upgrade pip
    & python -m pip install -r $PSScriptRoot/requirements.txt
}
finally {
    $env:PATH = $tempPath
}
