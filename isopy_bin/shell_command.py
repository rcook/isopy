from isopy_lib.env import exec_environment
import os


def do_shell(ctx, env):
    with exec_environment(ctx=ctx, env=env) as (python_bin_dir, e):
        print(
            f"Python shell for environment {env}; Python is at {python_bin_dir}")
        print(f"Type \"exit\" to return to parent shell")
        shell = os.getenv("SHELL")
        os.execle(shell, shell, e)
