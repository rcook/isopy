from isopy_lib.env import get_current_env_config
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path
import os


WRAPPER_TEMPLATE = """#!/bin/bash
set -euo pipefail
PATH={bin_dir}:$PATH \\
  PYTHONPATH={base_dir} \\
  exec python3 {script_path} "$@"
"""


def do_wrap(ctx, wrapper_path, script_path, base_dir):
    env_config = get_current_env_config(ctx=ctx)
    bin_dir = dir_path(env_config.path, "..", env_config.python_dir, "bin")
    wrapper = WRAPPER_TEMPLATE.format(
        bin_dir=bin_dir,
        base_dir=base_dir,
        script_path=script_path)

    try:
        with open(wrapper_path, "xt") as f:
            f.write(wrapper)
    except FileExistsError as e:
        raise ReportableError(
            f"File {wrapper_path} already exists; "
            "pass --force to overwrite") from e

    os.chmod(wrapper_path, mode=0o755)
