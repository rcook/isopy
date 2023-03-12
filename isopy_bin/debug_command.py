from collections import namedtuple
from isopy_lib.checksum import CHECKSUM_DIR
from isopy_lib.xprint import xprint
import colorama
import glob
import os
import sys


class ProgramInfo(namedtuple("ProgramInfo", ["paths", "python_paths", "cwd", "cache_dir", "checksum_dir", "checksum_paths", "frozen", "app_path"])):
    @staticmethod
    def get(ctx):
        frozen = getattr(sys, "frozen", False)
        if frozen:
            # If the application is run as a bundle, the PyInstaller bootloader
            # extends the sys module by a flag frozen=True and sets the app
            # path into variable _MEIPASS.
            app_path = sys._MEIPASS
        else:
            app_path = os.path.dirname(os.path.abspath(__file__))

        return ProgramInfo(
            paths=os.getenv("PATH").split(":"),
            python_paths=sys.path,
            cwd=ctx.cwd,
            cache_dir=ctx.cache_dir,
            checksum_dir=CHECKSUM_DIR,
            checksum_paths=glob.glob(f"{CHECKSUM_DIR}/*.sha256sums"),
            frozen=frozen,
            app_path=app_path)


def do_debug(ctx):
    def debug_print(*args, **kwargs):
        xprint(colorama.Fore.YELLOW, *args, **kwargs)

    program_info = ProgramInfo.get(ctx=ctx)

    debug_print("System search paths:")
    for p in program_info.paths:
        debug_print(f"  {p}")

    debug_print("Python paths:")
    for p in program_info.python_paths:
        debug_print(f"  {p}")

    debug_print(f"cwd={program_info.cwd}")
    debug_print(f"cache_dir={program_info.cache_dir}")
    debug_print(f"checksum_dir={program_info.checksum_dir}")

    debug_print(f"checksums:")
    for f in program_info.checksum_paths:
        debug_print(f"  {f}")

    debug_print(f"frozen={program_info.frozen}")
    debug_print(f"app_path={program_info.app_path}")
    os.system(f"tree {program_info.app_path}")
