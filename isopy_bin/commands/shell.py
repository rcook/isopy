from isopy_lib.env import UseInfo, get_env_config
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path
from isopy_lib.platform import PLATFORM
from isopy_lib.xprint import xprint
import colorama
import os


ISOPY_ENV_VAR_NAME = "ISOPY_ENV"


def do_shell(ctx, env, prune_paths):
    if os.getenv(ISOPY_ENV_VAR_NAME) is not None:
        raise ReportableError(
            "You are already in an active isopy shell")

    if env is None:
        use_info = UseInfo.find(ctx=ctx)
        if use_info is not None:
            env = use_info.env

    env_config = get_env_config(ctx=ctx, env=env)

    label = env_config.name or env_config.dir_config_path
    xprint(
        colorama.Fore.LIGHTYELLOW_EX,
        f"Python shell for environment {label}")
    xprint(
        colorama.Fore.YELLOW,
        "Type \"exit\" to return to parent shell")

    python_dir = env_config.make_python_dir(ctx=ctx)
    path_dirs = [
        dir_path(python_dir, d)
        for d in PLATFORM.python_bin_dirs
    ]
    extra_env = {ISOPY_ENV_VAR_NAME: label}
    PLATFORM.exec(
        path_dirs=path_dirs,
        extra_env=extra_env,
        prune_paths=prune_paths)
