from isopy_lib.env import get_env_config
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path
from isopy_lib.platform import LINUX, MACOS, PLATFORM, WINDOWS
import os


BASH_WRAPPER_TEMPLATE = """#!/bin/bash
set -euo pipefail
{path_env} \\
  PYTHONPATH={base_dir} \\
  exec {python_executable_name} {script_path} "$@"
"""

POWERSHELL_WRAPPER_TEMPLATE = """
#Requires -Version 5
[CmdletBinding()]
param(
    [Parameter(Mandatory = $false, Position = 0, ValueFromRemainingArguments = $true)]
    [object[]] $Arguments
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
try {{
    $tempPath = $env:Path
    $tempPythonPath = $env:PYTHONPATH
    {path_env}
    $env:PYTHONPATH = '{base_dir}'
    & {python_executable_name} "{script_path}" $Arguments
}}
finally {{
    $env:PYTHONPATH = $tempPythonPath
    $env:Path = $tempPath
}}
"""


def do_wrap(ctx, env, wrapper_path, script_path, base_dir, force):
    env_config = get_env_config(ctx=ctx, env=env)

    if PLATFORM in [LINUX, MACOS]:
        wrapper_template = BASH_WRAPPER_TEMPLATE
    elif PLATFORM == WINDOWS:
        wrapper_template = POWERSHELL_WRAPPER_TEMPLATE
    else:
        raise NotImplementedError(f"Unsupported platform {PLATFORM}")

    wrapper = wrapper_template.format(
        path_env=PLATFORM.path_env(env_config=env_config, export=False),
        base_dir=base_dir,
        python_executable_name=PLATFORM.python_executable_name,
        script_path=script_path)

    try:
        with open(wrapper_path, "wt" if force else "xt") as f:
            f.write(wrapper)
    except FileExistsError as e:
        raise ReportableError(
            f"File {wrapper_path} already exists; "
            "pass --force to overwrite") from e

    os.chmod(wrapper_path, mode=0o755)
