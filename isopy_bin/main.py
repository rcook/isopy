from isopy_bin.commands.available import do_available
from isopy_bin.commands.create import do_create
from isopy_bin.commands.debug import do_debug
from isopy_bin.commands.download import do_download
from isopy_bin.commands.downloaded import do_downloaded
from isopy_bin.commands.exec import do_exec
from isopy_bin.commands.info import do_info
from isopy_bin.commands.init import do_init
from isopy_bin.commands.list import do_list
from isopy_bin.commands.new import do_new
from isopy_bin.commands.shell import do_shell
from isopy_bin.commands.wrap import do_wrap
from isopy_lib.asset import AssetFilter
from isopy_lib.cli import \
    add_cache_dir_arg, \
    add_env_arg, \
    add_env_positional_arg, \
    add_log_level_arg, \
    add_python_version_arg, \
    add_python_version_positional_arg, \
    add_refresh_arg, \
    add_subcommand, \
    add_tag_name_arg, \
    auto_description
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
    def dir_path_type(s):
        return dir_path(cwd, s)

    def file_path_type(s):
        return file_path(cwd, s)

    default_cache_dir = get_default_cache_dir()

    def add_common_args(parser):
        add_log_level_arg(parser=parser)
        add_cache_dir_arg(parser=parser, default=default_cache_dir)

    extra_info = f"Using Python {sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"
    if not ProgramInfo.get(cwd=cwd, cache_dir=default_cache_dir).frozen:
        extra_info += f" ({shutil.which(PLATFORM.python_executable_name)})"

    parser = argparse.ArgumentParser(
        prog="isopy",
        description="Isolated Python Tool",
        epilog=extra_info)

    subparsers = parser.add_subparsers(required=True)

    p = add_subcommand(
        subparsers,
        "debug",
        **auto_description("show debugging information"),
        func=lambda ctx, args: do_debug(ctx=ctx))
    add_common_args(parser=p)

    p = add_subcommand(
        subparsers,
        "available",
        **auto_description("list available Python versions"),
        func=lambda ctx, args: do_available(
            ctx=ctx,
            asset_filter=AssetFilter.default(
                tag_name=args.tag_name,
                python_version=args.python_version),
            refresh=args.refresh))
    add_common_args(parser=p)
    add_tag_name_arg(parser=p)
    add_python_version_arg(parser=p)
    add_refresh_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "download",
        **auto_description("download Python package"),
        func=lambda ctx, args: do_download(
            ctx=ctx,
            asset_filter=AssetFilter.default(
                tag_name=args.tag_name,
                python_version=args.python_version)))
    add_common_args(parser=p)
    add_python_version_positional_arg(parser=p)
    add_tag_name_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "downloaded",
        **auto_description("show download Python packages"),
        func=lambda ctx, args: do_downloaded(ctx))
    add_common_args(parser=p)

    p = add_subcommand(
        subparsers,
        "list",
        **auto_description("list environments"),
        func=lambda ctx, args: do_list(ctx=ctx))
    add_common_args(parser=p)

    p = add_subcommand(
        subparsers,
        "new",
        **auto_description("create project configuration"),
        func=lambda ctx, args: do_new(
            ctx=ctx,
            asset_filter=AssetFilter.default(
                tag_name=args.tag_name,
                python_version=args.python_version)))
    add_common_args(parser=p)
    add_python_version_positional_arg(parser=p)
    add_tag_name_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "init",
        **auto_description("initialize environment corresponding to current project"),
        func=lambda ctx, args: do_init(ctx=ctx))
    add_common_args(parser=p)

    p = add_subcommand(
        subparsers,
        "create",
        **auto_description("initialize named environment"),
        func=lambda ctx, args: do_create(
            ctx=ctx,
            env=args.env,
            asset_filter=AssetFilter.default(
                tag_name=args.tag_name,
                python_version=args.python_version)))
    add_common_args(parser=p)
    add_env_positional_arg(parser=p)
    add_python_version_positional_arg(parser=p)
    add_tag_name_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "info",
        **auto_description("information about current environment"),
        func=lambda ctx, args: do_info(ctx=ctx, env=args.env))
    add_common_args(parser=p)
    add_env_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "shell",
        **auto_description("open shell"),
        func=lambda ctx, args: do_shell(ctx=ctx, env=args.env))
    add_common_args(parser=p)
    add_env_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "exec",
        **auto_description("run command in shell"),
        func=lambda ctx, args: do_exec(ctx=ctx, env=args.env, command=args.command))
    add_common_args(parser=p)
    add_env_arg(parser=p)
    p.add_argument(
        "command",
        nargs=argparse.REMAINDER,
        metavar="COMMAND",
        help="command to run and its arguments")

    p = add_subcommand(
        subparsers,
        "wrap",
        **auto_description("generate shell wrapper for Python script"),
        func=lambda ctx, args: do_wrap(
            ctx=ctx,
            env=args.env,
            wrapper_path=args.wrapper_path,
            script_path=args.script_path,
            base_dir=args.base_dir))
    add_common_args(parser=p)
    add_env_arg(parser=p)
    p.add_argument(
        "wrapper_path",
        metavar="WRAPPER_PATH",
        type=file_path_type,
        help="path to output wrapper script")
    p.add_argument(
        "script_path",
        metavar="SCRIPT_PATH",
        type=file_path_type,
        help="path to Python script")
    p.add_argument(
        "base_dir",
        metavar="BASE_DIR",
        type=dir_path_type,
        help="path to base directory")

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
