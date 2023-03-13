from isopy_lib.env import get_env_config
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path
from isopy_lib.platform import Platform, shell_execute
from isopy_lib.xprint import xprint
import colorama
import os


ISOPY_ENV_VAR_NAME = "ISOPY_ENV"


def do_shell(ctx, env):
    if os.getenv(ISOPY_ENV_VAR_NAME) is not None:
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

    python_dir = env_config.make_python_dir(ctx=ctx)
    c = Platform.current()
    path_dirs = [
        dir_path(python_dir, d)
        for d in c.bin_dirs
    ]
    extra_env = {ISOPY_ENV_VAR_NAME: label}
    shell_execute(path_dirs=path_dirs, extra_env=extra_env)
