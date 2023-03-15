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
Usage: setup [-h] [--force] [LAUNCHER_PATH]

  LAUNCHER_PATH  path to launcher script
  --force        force overwrite of launcher script if it already exists

Visit https://rcook.github.io/isopy/ for more information
"@
}

function New-TemporaryDirectory {
  $tempDir = [System.IO.Path]::GetTempPath()
  [string] $name = [System.Guid]::NewGuid()
  New-Item -ItemType Directory -Path (Join-Path -Path $tempDir -ChildPath $name)
}

$isopyBranch = 'main'
$isopyUrl = "https://github.com/rcook/isopy/archive/refs/heads/$isopyBranch.zip"
$isopyDir = Join-Path -Path $env:USERPROFILE -ChildPath .isopy
$isopyBinDir = Join-Path -Path $isopyDir -ChildPath bin
$isopySrcDir = Join-Path -Path $isopyDir -ChildPath src
$isopyAssetsDir = Join-Path -Path $isopyDir -ChildPath assets
$isopyBranchDir = Join-Path -Path $isopySrcDir -ChildPath $isopyBranch
$isopyEnvDir = Join-Path -Path $isopyDir -ChildPath envs\isopy
$isopyEnvPath = Join-Path -Path $isopyEnvDir -ChildPath env.yaml
$isopyEnvPythonDir = Join-Path -Path $isopyEnvDir -ChildPath python
$isopyEnvScriptsDir = Join-Path -Path $isopyEnvPythonDir -ChildPath Scripts

$argv = @() + $Arguments
if ($argv -in @('-h', '--help')) {
  Show-Usage
  exit 0
}

# Default behaviour when called from iwr/iex
if (($argv.Length -eq 1) -and ($null -eq $argv[0])) {
  $launcherPath = Join-Path -Path $isopyBinDir -ChildPath isopy.cmd
  $force = $true
}
else {
  $launcherPath = $null
  $force = $null
}

while ($null -ne $argv) {
  [string] $arg, [string[]] $argv = $argv
  if ($arg.Length -eq 0) { break }

  if ($arg -in @('-f', '--force')) {
    if ($null -ne $force) {
      Write-Host -Object "--force already specified`n"
      Show-Usage
      exit 1
    }
    $force = $true
    continue
  }

  if ($arg.StartsWith('-')) {
    Write-Host -Object "Invalid option $arg`n"
    Show-Usage
    exit 1
  }

  if ($null -ne $launcherPath) {
    Write-Host -Object "Launcher path already specified`n"
    Show-Usage
    exit 1
  }

  [string] $launcherPath = Resolve-Path -Path $arg -ErrorAction SilentlyContinue -ErrorVariable _frperror
  if (-not $launcherPath) {
    $launcherPath = $_frperror[0].TargetObject
  }
}

if ((-not $force) -and ($null -ne $launcherPath) -and (Test-Path -Path $launcherPath)) {
  Write-Host -Object "Launcher $launcherPath already exists; specify --force to overwrite it"
  exit 1
}

$tempDir = New-TemporaryDirectory
if (-not (Test-Path -Path $isopyBranchDir)) {
  try {
    $tempZipDir = Join-Path -Path $tempDir -ChildPath isopy-$isopyBranch
    $tempZipPath = Join-Path -Path $tempDir -ChildPath isopy-$isopyBranch.zip
    Invoke-WebRequest -Uri $isopyUrl -OutFile $tempZipPath
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
if (-not $m.Success) { throw 'Regex match failed' }
$pythonVersion = $m.Groups[1].Value
$m = [regex]::Match($c, "tag: '?([^\s']+)'?")
if (-not $m.Success) { throw 'Regex match failed' }
$tag = $m.Groups[1].Value
$pythonBaseName = "cpython-$pythonVersion+$tag"
$pythonFileName = "$pythonBaseName-x86_64-pc-windows-msvc-shared-install_only.tar.gz"

$pythonUrl = "https://github.com/indygreg/python-build-standalone/releases/download/$tag/$pythonFileName"
$pythonPath = Join-Path -Path $isopyAssetsDir -ChildPath $pythonFileName

New-Item -ItemType Directory -Path $isopyBinDir -Force | Out-Null
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

$launcher = @"
@echo off
setlocal
set PATH=$isopyEnvPythonDir;$isopyEnvScriptsDir;%PATH%
set PYTHONPATH=$isopyBranchDir
python "$isopyBranchDir\isopy_bin\main.py" %*
"@

Set-Content -Path $launcherPath -Value $launcher
$launcherDir = (Get-Item -Path $launcherPath).Directory.FullName
Write-Host -Object @"

--------------------------------------------------
Launcher script generated at $launcherPath
To access isopy globally, either add $launcherDir to the beginning of
%PATH% or copy $launcherPath to a directory already on %PATH%.
--------------------------------------------------
Visit https://rcook.github.io/isopy/ for more information
--------------------------------------------------
"@
