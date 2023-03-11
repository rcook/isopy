from isopy_lib.errors import ReportableError
import os


WRAPPER_TEMPLATE = """#!/bin/bash
set -euo pipefail
PATH=$HOME/.isopy/env/{env}/cpython-3.11.1+20230116/bin:$PATH \\
  PYTHONPATH={base_dir} \\
  exec python3 {script_path} "$@"
"""


def do_wrap(ctx, env, wrapper_path, script_path, base_dir, force):
    wrapper = WRAPPER_TEMPLATE.format(
        env=env,
        script_path=script_path,
        base_dir=base_dir)
    try:
        with open(wrapper_path, "wt" if force else "xt") as f:
            f.write(wrapper)
    except FileExistsError as e:
        raise ReportableError(
            f"File {wrapper_path} already exists; "
            "pass --force to overwrite") from e

    os.chmod(wrapper_path, mode=0o755)
