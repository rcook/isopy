from isopy_lib.env import EnvConfig
from isopy_lib.pretty import show_table


def do_list(ctx):
    env_configs = EnvConfig.load_all(ctx=ctx)
    if len(env_configs) > 0:
        show_table(items=env_configs)
    else:
        print("There are no environments yet")
