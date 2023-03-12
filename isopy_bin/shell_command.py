from isopy_lib.env import get_current_env_config
from isopy_lib.xprint import xprint
import colorama
import os


def do_shell(ctx):
    env_config = get_current_env_config(ctx=ctx)
    xprint(
        colorama.Fore.LIGHTYELLOW_EX,
        f"Python shell for environment {env_config.dir_config_path}")
    xprint(
        colorama.Fore.YELLOW,
        "Type \"exit\" to return to parent shell")

    shell = os.getenv("SHELL")
    os.execlpe(shell, shell, env_config.get_environment(ctx=ctx))
