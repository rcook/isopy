from isopy_lib.env import DirConfig, EnvConfig
from isopy_lib.errors import ReportableError
from isopy_lib.platform import Platform
from isopy_lib.xprint import xprint
import colorama
import os


def do_shell(ctx):
    if Platform.current() not in [Platform.LINUX, Platform.MACOS]:
        raise NotImplementedError(f"Not supported for this platform yet")

    dir_config = DirConfig.find(ctx=ctx)
    if dir_config is None:
        raise ReportableError(
            f"No isopy configuration found for directory {ctx.cwd}; "
            "consider creating one with \"isopy new\"")

    env_config = EnvConfig.find(ctx=ctx, dir_config_path=dir_config.path)
    if env_config is None:
        raise ReportableError(
            f"No environment initialized for {dir_config.path}")

    e = env_config.get_environment(ctx=ctx)
    xprint(
        colorama.Fore.LIGHTYELLOW_EX,
        f"Python shell for environment {dir_config.path}")
    xprint(
        colorama.Fore.YELLOW,
        "Type \"exit\" to return to parent shell")
    shell = os.getenv("SHELL")
    os.execlpe(shell, shell, e)
