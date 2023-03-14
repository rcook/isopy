@echo off
setlocal
set PATH=%USERPROFILE%\.isopy\envs\isopy\python;%USERPROFILE%\.isopy\envs\isopy\python\Scripts;%PATH%
set PYTHONPATH=%~dp0
python %~dp0isopy_bin\main.py %*
