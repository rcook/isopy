from isopy_bin.arg_helper import ArgHelper
from isopy_bin.asset_subcommands import add_asset_subcommands
from isopy_bin.debug_subcommands import add_debug_subcommands
from isopy_bin.env_subcommands import add_env_subcommands
from isopy_bin.project_subcommands import add_project_subcommands
from isopy_bin.shell_subcommands import add_shell_subcommands
from isopy_lib.cli import add_cache_dir_arg, add_log_level_arg
from isopy_lib.context import Context
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path, file_path
from isopy_lib.platform import PLATFORM
from isopy_lib.program_info import ProgramInfo, get_default_cache_dir
from isopy_lib.xprint import xprint
import argparse
import colorama
import logging
import os
import shutil
import sys


def main(cwd, argv):
    default_cache_dir = get_default_cache_dir()

    class _ArgHelper(ArgHelper):
        def dir_path_type(self, s):
            return dir_path(cwd, s)

        def file_path_type(self, s):
            return file_path(cwd, s)

        def add_common_args(self, parser):
            add_log_level_arg(parser=parser)
            add_cache_dir_arg(parser=parser, default=default_cache_dir)

    h = _ArgHelper()

    python_version_str = ".".join([str(s) for s in [
        sys.version_info.major,
        sys.version_info.minor,
        sys.version_info.micro
    ]])
    extra_info = f"Using Python {python_version_str}"
    if not ProgramInfo.get(cwd=cwd, cache_dir=default_cache_dir).frozen:
        extra_info += f" ({shutil.which(PLATFORM.python_executable_name)})"

    parser = argparse.ArgumentParser(
        prog="isopy",
        description="Isolated Python Tool",
        epilog=extra_info)

    subparsers = parser.add_subparsers(required=True)
    add_debug_subcommands(helper=h, subparsers=subparsers)
    add_asset_subcommands(helper=h, subparsers=subparsers)
    add_env_subcommands(helper=h, subparsers=subparsers)
    add_shell_subcommands(helper=h, subparsers=subparsers)
    add_project_subcommands(helper=h, subparsers=subparsers)

    args = parser.parse_args(argv)

    logging.basicConfig(
        format="%(asctime)s %(levelname)s %(message)s")
    logger = logging.getLogger(__name__)
    logger.setLevel(logging.getLevelNamesMapping()[args.log_level.upper()])

    args.func(
        ctx=Context(
            cwd=cwd,
            logger=logger,
            cache_dir=args.cache_dir),
        args=args)


if __name__ == "__main__":
    colorama.init()
    try:
        main(cwd=os.getcwd(), argv=sys.argv[1:])
    except ReportableError as e:
        xprint(colorama.Fore.RED, str(e), file=sys.stderr)
        exit(1)
