#Requires -Version 5
[CmdletBinding()]
param(
    [Parameter(Mandatory = $false, Position = 0, ValueFromRemainingArguments = $true)]
    [object[]] $Arguments
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$pythonDir = Join-Path -Path $env:USERPROFILE -ChildPath .isopy\envs\isopy\python
$thisDir = $PSScriptRoot
try {
    $tempPath = $env:Path
    $tempPythonPath = $env:PYTHONPATH
    $env:PATH = $pythonDir + ';' + "$pythonDir\Scripts" + ';' + $env:Path
    $env:PYTHONPATH = $thisDir
    & python $thisDir\isopy_bin\main.py $Arguments
}
finally {
    $env:PYTHONPATH = $tempPythonPath
    $env:Path = $tempPath
}
