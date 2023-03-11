from isopy_lib.env import EnvInfo
from isopy_lib.pretty import show_table


def do_list(ctx):
    env_infos = EnvInfo.load_all(cache_dir=ctx.cache_dir)
    if len(env_infos) > 0:
        show_table(items=env_infos)
    else:
        print("There are no environments yet!")
