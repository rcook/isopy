from isopy_lib.env import EnvConfig
from isopy_lib.fs import get_home_dir, get_home_dir_meta
from isopy_lib.pretty import show_table
import os


def transform(x):
    home_dir_meta = get_home_dir_meta()
    home_dir = get_home_dir()

    def truncate_path(s):
        if s == home_dir:
            return home_dir_meta
        if s.startswith(home_dir + os.sep):
            return home_dir_meta + s[len(home_dir):]
        return s

    return {
        "path": truncate_path(x.path),
        "name_or_dir": truncate_path(x.dir_config_path) if x.name is None else x.name,
        "tag": x.tag_name,
        "ver": x.python_version
    }


def do_list(ctx):
    env_configs = EnvConfig.load_all(ctx=ctx)
    if len(env_configs) > 0:
        show_table(items=[transform(x) for x in env_configs])
    else:
        print("There are no environments yet")
