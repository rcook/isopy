from isopy_lib.asset import assets_dir as __assets_dir
from isopy_lib.version import Version
from isopy_lib.xprint import xprint
from operator import itemgetter
import colorama
import os


CPYTHON_PREFIX = "cpython-"
CPYTHON_PREFIX_LEN = len(CPYTHON_PREFIX)


def do_downloaded(ctx):
    assets_dir = __assets_dir(ctx.cache_dir)
    if os.path.isdir(assets_dir):
        ps = []
        for f in os.listdir(assets_dir):
            if f.startswith(CPYTHON_PREFIX):
                idx = f.index("+", CPYTHON_PREFIX_LEN)
                version = Version.parse(f[CPYTHON_PREFIX_LEN:idx])
                ps.append((f, version))

        for f, version in sorted(ps, key=itemgetter(1), reverse=True):
            xprint(colorama.Fore.YELLOW, f, " ", version)
    else:
        print("There are no downloads yet!")
