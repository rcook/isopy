from isopy_lib.env import EnvConfig
from isopy_lib.fs import dir_path
from isopy_lib.platform import PLATFORM
from isopy_lib.pretty import show_table
import os


def transform(env_config, detailed):
    def truncate_path(s):
        if s == PLATFORM.home_dir:
            return PLATFORM.home_dir_meta
        if s.startswith(PLATFORM.home_dir + os.sep):
            return PLATFORM.home_dir_meta + s[len(PLATFORM.home_dir):]
        return s

    if detailed:
        name_or_dir = truncate_path(env_config.dir_config_path) \
            if env_config.name is None \
            else env_config.name
        return {
            "path": truncate_path(env_config.path),
            "name_or_dir": name_or_dir,
            "tag": env_config.tag,
            "ver": env_config.python_version,
            "PATH": PLATFORM.path_env(env_config=env_config)
        }
    else:
        return {
            "name_or_dir": truncate_path(x.dir_config_path) if x.name is None else x.name,
            "tag": x.tag,
            "ver": x.python_version
        }


def do_list(ctx, detailed):
    env_configs = EnvConfig.load_all(ctx=ctx)
    if len(env_configs) > 0:
        show_table(items=[transform(x, detailed) for x in env_configs])
    else:
        print("There are no environments yet")
