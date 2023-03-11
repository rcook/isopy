import os


WRAPPER_TEMPLATE = """#!/bin/bash
set -euo pipefail
PATH=$HOME/.isopy/env/{env}/cpython-3.11.1+20230116/bin:$PATH \\
  PYTHONPATH={base_dir} \\
  python3 {script_path} "$@"
"""


def do_wrap(ctx, env, wrapper_path, script_path, base_dir):
    wrapper = WRAPPER_TEMPLATE.format(
        env=env,
        script_path=script_path,
        base_dir=base_dir)
    with open(wrapper_path, "xt") as f:
        f.write(wrapper)
    os.chmod(wrapper_path, mode=0o755)
