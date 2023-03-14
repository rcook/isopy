@echo off
setlocal
powershell -NoProfile %~dp0bootstrap.ps1 %*
