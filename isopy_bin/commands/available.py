from isopy_lib.asset import get_assets
from isopy_lib.pretty import show_table


def transform(x):
    return {
        "os": x.os,
        "arch": x.arch,
        "tag": x.tag,
        "ver": x.python_version
    }


def do_available(ctx, asset_filter, refresh):
    assets = get_assets(ctx=ctx, asset_filter=asset_filter, refresh=refresh)
    if len(assets) > 0:
        show_table(items=[transform(x) for x in assets])
    else:
        print("There are no assets yet")
