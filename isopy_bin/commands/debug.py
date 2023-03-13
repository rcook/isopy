from isopy_lib.program_info import ProgramInfo
from isopy_lib.xprint import xprint
import colorama
import os


def do_debug(ctx):
    def show(*args, **kwargs):
        xprint(colorama.Fore.YELLOW, *args, **kwargs)

    def show_value(key, value):
        xprint(
            colorama.Fore.YELLOW,
            key,
            colorama.Fore.WHITE,
            ": ",
            colorama.Fore.LIGHTWHITE_EX, value)

    program_info = ProgramInfo.get(cwd=ctx.cwd, cache_dir=ctx.cache_dir)

    show("System search paths:")
    for p in program_info.paths:
        show(f"  {p}")

    show("Python paths:")
    for p in program_info.python_paths:
        show(f"  {p}")

    show_value("cwd", program_info.cwd)
    show_value("cache_dir", program_info.cache_dir)
    show_value("checksum_dir", program_info.checksum_dir)

    show(f"checksums:")
    for f in program_info.checksum_paths:
        show(f"  {f}")

    show_value("frozen", program_info.frozen)
    show_value("app_path", program_info.app_path)
    os.system(f"tree {program_info.app_path}")
