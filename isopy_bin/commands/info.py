from isopy_lib.env import get_env_config
from isopy_lib.xprint import xprint
import colorama
import os


def do_info(ctx, env):
    def show(*args, **kwargs):
        xprint(colorama.Fore.YELLOW, *args, **kwargs)

    def show_value(key, value):
        xprint(
            colorama.Fore.YELLOW,
            key,
            colorama.Fore.WHITE,
            ": ",
            colorama.Fore.LIGHTWHITE_EX, value)

    env_config = get_env_config(ctx=ctx, env=env)

    if env_config.name is not None:
        show_value("name", env_config.name)

    if env_config.dir_config_path is not None:
        show_value("dir_config_path", env_config.dir_config_path)

    show_value("path", env_config.path)
    show_value("tag", env_config.tag)
    show_value("python_version", env_config.python_version)
    show_value("python_dir", env_config.python_dir)
