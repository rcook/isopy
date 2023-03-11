from isopy_bin.available_command import do_available
from isopy_bin.list_command import do_list
from isopy_bin.new_command import do_new
from isopy_bin.shell_command import do_shell
from isopy_lib.asset import AssetFilter
from isopy_lib.cli import add_env_positional_arg, add_log_level_arg, add_python_version_positional_arg, auto_description
from isopy_lib.context import Context
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path
from isopy_lib.version import Version
from isopy_lib.xprint import xprint
import argparse
import colorama
import json
import logging
import os
import sys


def main(cwd, argv):
    default_cache_dir = os.path.expanduser("~/.isopy")

    def add_subcommand(subparsers, *args, func, **kwargs):
        parser = subparsers.add_parser(*args, **kwargs)
        parser.set_defaults(func=func)
        return parser

    def add_cache_dir_arg(parser):
        parser.add_argument(
            "--cache-dir",
            "-c",
            metavar="CACHE_DIR",
            default=default_cache_dir,
            help=f"cache directory (default: {default_cache_dir})")

    def add_tag_name_arg(parser):
        parser.add_argument(
            "--tag-name",
            "-t",
            metavar="TAG_NAME",
            type=str,
            required=False,
            help="tag name")

    def add_common_args(parser):
        add_log_level_arg(parser=parser)
        add_cache_dir_arg(parser=parser)

    parser = argparse.ArgumentParser(
        prog="isopy",
        description="Isolated Python Tool")

    subparsers = parser.add_subparsers(required=True)

    p = add_subcommand(
        subparsers,
        "list",
        **auto_description("list environments"),
        func=lambda ctx, args: do_list(ctx=ctx))
    add_common_args(parser=p)

    p = add_subcommand(
        subparsers,
        "download",
        **auto_description("download Python package"),
        func=lambda logger, args: do_download(
            logger=logger,
            cache_dir=args.cache_dir,
            env=args.env,
            asset_filter=AssetFilter.default(
                tag_name=args.tag_name,
                python_version=args.python_version)))
    add_common_args(parser=p)
    add_python_version_positional_arg(parser=p)
    add_tag_name_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "new",
        **auto_description("create new isolated Python environment"),
        func=lambda ctx, args: do_new(
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
        "shell",
        **auto_description("open shell in Python environment"),
        func=lambda ctx, args: do_shell(ctx=ctx, env=args.env))
    add_common_args(parser=p)
    add_env_positional_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "available",
        **auto_description("list available Python versions"),
        func=lambda ctx, args: do_available(
            ctx=ctx,
            asset_filter=AssetFilter.default(
                tag_name=args.tag_name,
                python_version=args.python_version)))
    add_common_args(parser=p)
    add_tag_name_arg(parser=p)
    p.add_argument(
        "--python-version",
        "-v",
        metavar="PYTHON_VERSION",
        type=Version.parse,
        required=False,
        help="Python version")

    args = parser.parse_args(argv)

    logging.basicConfig(
        format="%(asctime)s %(levelname)s %(message)s")
    logger = logging.getLogger(__name__)
    logger.setLevel(logging.getLevelNamesMapping()[args.log_level.upper()])
    args.func(ctx=Context(logger=logger, cache_dir=args.cache_dir), args=args)


if __name__ == "__main__":
    colorama.init()
    try:
        main(cwd=os.getcwd(), argv=sys.argv[1:])
    except ReportableError as e:
        xprint(colorama.Fore.RED, str(e), file=sys.stderr)
        exit(1)
