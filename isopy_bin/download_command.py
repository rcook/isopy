from isopy_lib.asset import get_asset


def do_download(ctx, asset_filter):
    get_asset(
        ctx=ctx,
        asset_filter=asset_filter).download(ctx=ctx)
