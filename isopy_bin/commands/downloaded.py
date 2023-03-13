from isopy_lib.asset import assets_dir as assets_dir__
from isopy_lib.fs import file_path
from isopy_lib.version import Version
from isopy_lib.pretty import show_table
from operator import itemgetter
import os


CPYTHON_PREFIX = "cpython-"
CPYTHON_PREFIX_LEN = len(CPYTHON_PREFIX)


def get_downloads(ctx):
    items = []
    assets_dir = assets_dir__(ctx.cache_dir)
    if os.path.isdir(assets_dir):
        for f in os.listdir(assets_dir):
            if f.startswith(CPYTHON_PREFIX):
                idx = f.index("+", CPYTHON_PREFIX_LEN)
                python_version = Version.parse(f[CPYTHON_PREFIX_LEN:idx])
                p = file_path(assets_dir, f)
                items.append({
                    "file": f,
                    "ver": python_version,
                    "size": os.path.getsize(p)
                })
    return sorted(items, key=itemgetter("ver"), reverse=True)


def do_downloaded(ctx):
    downloads = get_downloads(ctx=ctx)
    if len(downloads) > 0:
        show_table(items=downloads)
    else:
        print("There are no downloads yet")