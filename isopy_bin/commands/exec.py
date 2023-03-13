from isopy_lib.env import get_env_config
from isopy_lib.fs import dir_path
from isopy_lib.platform import Platform, exec


def do_exec(ctx, env, command):
    env_config = get_env_config(ctx=ctx, env=env)
    python_dir = env_config.make_python_dir(ctx=ctx)
    c = Platform.current()
    path_dirs = [
        dir_path(python_dir, d)
        for d in c.bin_dirs
    ]
    exec(path_dirs=path_dirs, command=command)
