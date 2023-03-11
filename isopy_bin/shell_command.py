from isopy_lib.env import exec_environment
from isopy_lib.xprint import xprint
import colorama
import os


def do_shell(ctx, env):
    with exec_environment(ctx=ctx, env=env) as (python_bin_dir, e):
        xprint(
            colorama.Fore.LIGHTYELLOW_EX,
            f"Python shell for environment {env}; Python is at {python_bin_dir}")
        xprint(
            colorama.Fore.YELLOW,
            "Type \"exit\" to return to parent shell")
        shell = os.getenv("SHELL")
        os.execlpe(shell, shell, e)
