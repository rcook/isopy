from isopy_lib.program_info import ProgramInfo
from isopy_lib.xprint import xprint
import colorama
import os


def do_debug(ctx):
    def debug_print(*args, **kwargs):
        xprint(colorama.Fore.YELLOW, *args, **kwargs)

    program_info = ProgramInfo.get(cwd=ctx.cwd, cache_dir=ctx.cache_dir)

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
