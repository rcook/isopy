from isopy_lib.checksum import CHECKSUM_DIR
from isopy_lib.xprint import xprint
import colorama
import os
import sys


def do_debug(ctx):
    def debug_print(*args, **kwargs):
        xprint(colorama.Fore.YELLOW, *args, **kwargs)

    debug_print("PATH:")
    for p in os.getenv("PATH").split(":"):
        debug_print(f"  {p}")

    debug_print("sys.path:")
    for p in sys.path:
        debug_print(f"  {p}")

    debug_print(f"cwd={ctx.cwd}")
    debug_print(f"cache_dir={ctx.cache_dir}")

    debug_print(f"checksum_dir={CHECKSUM_DIR}")
    debug_print(f"checksums:")
    for f in os.listdir(CHECKSUM_DIR):
        if f.endswith(".sha256sums"):
            debug_print(f"  {f}")

    frozen = getattr(sys, "frozen", False)
    debug_print(f"frozen={frozen}")

    if frozen:
        # If the application is run as a bundle, the PyInstaller bootloader
        # extends the sys module by a flag frozen=True and sets the app
        # path into variable _MEIPASS.
        app_path = sys._MEIPASS
    else:
        app_path = os.path.dirname(os.path.abspath(__file__))

    debug_print(f"app_path={app_path}")

    os.system(f"tree {app_path}")
