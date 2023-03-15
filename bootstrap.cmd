@echo off
setlocal
powershell -NoProfile iex %~dp0bootstrap.ps1; bootstrap %*
