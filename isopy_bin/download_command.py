from isopy_lib.asset import assets_dir, get_asset
from isopy_lib.fs import file_path
import os


def do_download(ctx, asset_filter):
    asset = get_asset(ctx=ctx, asset_filter=asset_filter)

    python_path = file_path(assets_dir(ctx.cache_dir), asset.name)
    if os.path.isfile(python_path):
        ctx.logger.debug(f"Package {python_path} is already available locally")
    else:
        ctx.logger.debug(
            f"Downloading {asset.browser_download_url} to {python_path}")
        asset.download(python_path)
