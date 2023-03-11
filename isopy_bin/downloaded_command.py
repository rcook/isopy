from isopy_lib.asset import assets_dir as assets_dir__
from isopy_lib.fs import file_path
from isopy_lib.version import Version
from isopy_lib.xprint import xprint
from operator import itemgetter
import colorama
import os


CPYTHON_PREFIX = "cpython-"
CPYTHON_PREFIX_LEN = len(CPYTHON_PREFIX)


def do_downloaded(ctx):
    assets_dir = assets_dir__(ctx.cache_dir)
    if os.path.isdir(assets_dir):
        items = []
        for f in os.listdir(assets_dir):
            if f.startswith(CPYTHON_PREFIX):
                idx = f.index("+", CPYTHON_PREFIX_LEN)
                version = Version.parse(f[CPYTHON_PREFIX_LEN:idx])
                p = file_path(assets_dir, f)
                items.append((f, version, os.path.getsize(p)))

        for f, version, size in sorted(items, key=itemgetter(1), reverse=True):
            xprint(colorama.Fore.YELLOW, f, " ", version, " ", f"{size} bytes")
    else:
        print("There are no downloads yet!")
