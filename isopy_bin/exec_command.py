from isopy_lib.env import get_env_config
import os


def do_exec(ctx, env, command):
    env_config = get_env_config(ctx=ctx, env=env)
    os.execlpe(command[0], *command, env_config.get_environment(ctx=ctx))
