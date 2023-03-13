#Requires -Version 5
[CmdletBinding()]
param(
    [Parameter(Mandatory = $false, Position = 0, ValueFromRemainingArguments = $true)]
    [object[]] $Arguments
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$pythonDir = 'C:\Users\rcook\.isopy\hashed\7f0ec6a91047ee431ff45f55a6914c9c\cpython-3.11.1+20230116'
$thisDir = $PSScriptRoot
try {
    $tempPath = $env:PATH
    $tempPythonPath = $env:PYTHONPATH
    $env:PATH = $pythonDir + ';' + $env:PATH
    $env:PYTHONPATH = $thisDir
    & python.exe $thisDir\isopy_bin\main.py $Arguments
}
finally {
    $env:PYTHONPATH = $tempPythonPath
    $env:PATH = $tempPath
}
