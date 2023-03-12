from isopy_lib.env import get_current_env_config
import os


def do_exec(ctx, command):
    env_config = get_current_env_config(ctx=ctx)
    os.execlpe(command[0], *command, env_config.get_environment(ctx=ctx))
