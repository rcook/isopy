import os


def do_exec(ctx, env, command):
    with exec_environment(ctx=ctx, env=env) as (_, e):
        os.execlpe(command[0], *command, e)
