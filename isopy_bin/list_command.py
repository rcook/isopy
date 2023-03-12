from isopy_lib.env import EnvManifest
from isopy_lib.pretty import show_table


def do_list(ctx):
    manifests = EnvManifest.load_all_from_cache(ctx=ctx)
    if len(manifests) > 0:
        show_table(items=manifests)
    else:
        print("There are no environments yet!")
