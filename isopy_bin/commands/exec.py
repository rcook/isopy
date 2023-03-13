from isopy_lib.env import UseInfo, get_env_config
from isopy_lib.fs import dir_path
from isopy_lib.platform import PLATFORM


def do_exec(ctx, env, command):
    if env is None:
        use_info = UseInfo.find(ctx=ctx)
        if use_info is not None:
            env = use_info.env

    env_config = get_env_config(ctx=ctx, env=env)
    python_dir = env_config.make_python_dir(ctx=ctx)
    path_dirs = [
        dir_path(python_dir, d)
        for d in PLATFORM.python_bin_dirs
    ]
    PLATFORM.exec(path_dirs=path_dirs, command=command)
