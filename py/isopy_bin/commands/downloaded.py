from isopy_lib.asset import AssetNameInfo, assets_dir as assets_dir__
from isopy_lib.fs import file_path
from isopy_lib.version import Version
from isopy_lib.pretty import show_table
from operator import itemgetter
import os


CPYTHON_PREFIX = "cpython-"
CPYTHON_PREFIX_LEN = len(CPYTHON_PREFIX)


def get_downloads(ctx, detailed):
    items = []
    assets_dir = assets_dir__(ctx.cache_dir)
    if os.path.isdir(assets_dir):
        for f in os.listdir(assets_dir):
            try:
                asset_name_info = AssetNameInfo.parse(f)
            except ValueError:
                continue

            p = file_path(assets_dir, f)
            if detailed:
                item = {
                    "file": f,
                    "ver": asset_name_info.python_version,
                    "size": os.path.getsize(p),
                    "create_command": f"\"isopy create MY_NAMED_ENVIRONMENT {asset_name_info.python_version} --tag {asset_name_info.tag}\""
                }
            else:
                item = {
                    "file": f,
                    "ver": asset_name_info.python_version,
                    "size": os.path.getsize(p)
                }

            items.append(item)
    return sorted(items, key=itemgetter("ver"), reverse=True)


def do_downloaded(ctx, detailed):
    downloads = get_downloads(ctx=ctx, detailed=detailed)
    if len(downloads) > 0:
        show_table(items=downloads)
    else:
        print("There are no downloads yet")
