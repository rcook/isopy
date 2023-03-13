from isopy_lib.env import get_env_config
from isopy_lib.errors import ReportableError
from isopy_lib.xprint import xprint
import colorama
import os


ISOPY_ENV_VAR_NAME = "ISOPY_ENV"


def do_shell(ctx, env):
    if os.getenv(ISOPY_ENV_VAR_NAME) is not None:
        raise ReportableError(
            "You are already in an active isopy environment")

    env_config = get_env_config(ctx=ctx, env=env)

    env_identifier = env_config.name or env_config.dir_config_path
    xprint(
        colorama.Fore.LIGHTYELLOW_EX,
        f"Python shell for environment {env_identifier}")
    xprint(
        colorama.Fore.YELLOW,
        "Type \"exit\" to return to parent shell")

    os.environ[ISOPY_ENV_VAR_NAME] = env_identifier
    env_config.execlpe(os.getenv("SHELL"), ctx=ctx)
