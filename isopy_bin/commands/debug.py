from isopy_lib.platform import PLATFORM, PYTHON_PROGRAMS
from isopy_lib.program_info import ProgramInfo
from isopy_lib.xprint import xprint
import colorama
import os
import shutil
import yaml


def do_debug(ctx, detailed):
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

    show("Platform information:")
    d = {
        k: v
        for k, v in PLATFORM._asdict().items()
        if k != "exec" and v is not None
    }
    for line in yaml.dump(d, sort_keys=True).splitlines():
        show(f"  {line}")

    if detailed:
        show("System search paths:")
        for p in program_info.paths:
            show(f"  {p}")

    if detailed:
        show("Python paths:")
        for p in program_info.python_paths:
            show(f"  {p}")

    show_value("cwd", program_info.cwd)
    show_value("cache_dir", program_info.cache_dir)
    show_value("checksum_dir", program_info.checksum_dir)

    if detailed:
        show(f"checksums:")
        for f in program_info.checksum_paths:
            show(f"  {f}")

    show_value("app_path", program_info.app_path)
    show_value("frozen", program_info.frozen)
    show_value(
        "bootstrapped_isopy",
        program_info.app_path.startswith(program_info.cache_dir + os.sep))

    if detailed:
        os.system(f"tree {program_info.app_path}")

    if detailed:
        # Deduplicate search path
        paths = []
        for p in program_info.paths:
            if p not in paths:
                paths.append(p)

        # Show Python programs accessible from search path
        for program in PYTHON_PROGRAMS:
            matches = [
                x
                for x in [
                    shutil.which(program, path=p)
                    for p in paths
                ]
                if x is not None
            ]
            if len(matches) > 0:
                xprint(colorama.Fore.GREEN, program)
                for m in matches:
                    show(f"  {m}")

    if not detailed:
        xprint(colorama.Fore.BLUE, "Pass --detail to get more detail")
