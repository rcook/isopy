from collections import namedtuple
from isopy_lib.checksum import CHECKSUM_DIR
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
            app_path = os.path.abspath(os.path.join(__file__, "..", ".."))

        return ProgramInfo(
            paths=os.getenv("PATH").split(":"),
            python_paths=sys.path,
            cwd=ctx.cwd,
            cache_dir=ctx.cache_dir,
            checksum_dir=CHECKSUM_DIR,
            checksum_paths=glob.glob(f"{CHECKSUM_DIR}/*.sha256sums"),
            frozen=frozen,
            app_path=app_path)
