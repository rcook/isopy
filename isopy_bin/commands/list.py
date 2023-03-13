from isopy_lib.env import EnvConfig
from isopy_lib.fs import dir_path
from isopy_lib.platform import PLATFORM
from isopy_lib.pretty import show_table
import os


def transform(x, detailed):
    def truncate_path(s):
        if s == PLATFORM.home_dir:
            return PLATFORM.home_dir_meta
        if s.startswith(PLATFORM.home_dir + os.sep):
            return PLATFORM.home_dir_meta + s[len(PLATFORM.home_dir):]
        return s

    if detailed:
        python_bin_dir = dir_path(x.path, "..", "bin")
        return {
            "path": truncate_path(x.path),
            "name_or_dir": truncate_path(x.dir_config_path) if x.name is None else x.name,
            "tag": x.tag,
            "ver": x.python_version,
            "PATH": f"export PATH={python_bin_dir}{os.pathsep}$PATH"
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
