from isopy_lib.env import get_env_config


def do_exec(ctx, env, command):
    get_env_config(ctx=ctx, env=env).execlpe(*command, ctx=ctx)
