from isopy_lib.env import get_env_config
from isopy_lib.errors import ReportableError
from isopy_lib.os import in_isopy_shell, start_isopy_shell
from isopy_lib.xprint import xprint
import colorama


def do_shell(ctx, env):
    if in_isopy_shell(ctx=ctx):
        raise ReportableError(
            "You are already in an active isopy shell")

    env_config = get_env_config(ctx=ctx, env=env)

    label = env_config.name or env_config.dir_config_path
    xprint(
        colorama.Fore.LIGHTYELLOW_EX,
        f"Python shell for environment {label}")
    xprint(
        colorama.Fore.YELLOW,
        "Type \"exit\" to return to parent shell")

    start_isopy_shell(ctx=ctx, env_config=env_config)
