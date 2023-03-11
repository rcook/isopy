from isopy_lib.asset import get_assets
from isopy_lib.pretty import show_table


def do_available(ctx, asset_filter):
    assets = get_assets(ctx=ctx, asset_filter=asset_filter)
    show_table(items=assets, fields=[
        "os",
        "arch",
        "tag_name",
        "python_version"
    ])
