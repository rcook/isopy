from isopy_bin.available_command import do_available
from isopy_bin.new_command import do_new
from isopy_lib.asset import AssetFilter
from isopy_lib.cli import add_env_positional_arg, add_python_version_positional_arg
from isopy_lib.context import Context
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path
from isopy_lib.manifest import EnvManifest
from isopy_lib.platform import Platform
from isopy_lib.version import Version
from isopy_lib.xprint import xprint
import argparse
import colorama
import json
import logging
import os
import sys


def do_list(logger, cache_dir):
    env_root_dir = env_root_dir(cache_dir=cache_dir)
    if os.path.exists(env_root_dir):
        for d in sorted(os.listdir(env_root_dir)):
            p = env_manifest_path(cache_dir=cache_dir, env=d)
            env_manifest = EnvManifest.load(p)
            print(f"{d}: {env_manifest.tag_name}, {env_manifest.python_version}")
    else:
        print("There are no environments yet!")


def do_shell(logger, cache_dir, env):
    if Platform.current() not in [Platform.LINUX, Platform.MACOS]:
        raise NotImplementedError(f"Not supported for this platform yet")

    with open(env_manifest_path(cache_dir=cache_dir, env=env), "rt") as f:
        manifest = json.load(f)

    python_dir = dir_path(
        env_dir(cache_dir=cache_dir, env=env),
        manifest["python_dir"])
    python_bin_dir = dir_path(python_dir, "bin")

    print(f"Python shell for environment {env}; Python is at {python_bin_dir}")
    print(f"Type \"exit\" to return to parent shell")

    e = dict(os.environ)
    temp = e.get("PATH")
    paths = [] if temp is None else temp.split(":")
    if python_bin_dir not in paths:
        e["PATH"] = ":".join([python_bin_dir] + paths)

    shell = os.getenv("SHELL")
    os.execle(shell, shell, e)


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
        add_cache_dir_arg(parser)

    parser = argparse.ArgumentParser(
        prog="isopy",
        description="Isolated Python Tool")

    subparsers = parser.add_subparsers(required=True)

    p = add_subcommand(
        subparsers,
        "list",
        help="list environments",
        description="List environments",
        func=lambda logger, args: do_list(
            logger=logger,
            cache_dir=args.cache_dir))
    add_cache_dir_arg(p)

    p = add_subcommand(
        subparsers,
        "download",
        help="download Python package",
        description="Download Python package",
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
        help="create new isolated Python environment",
        description="Create new isolated Python environment",
        func=lambda ctx, args: do_new(
            ctx=ctx,
            env=args.env,
            asset_filter=AssetFilter.default(
                tag_name=args.tag_name,
                python_version=args.python_version)))
    add_cache_dir_arg(parser=p)
    add_env_positional_arg(parser=p)
    add_python_version_positional_arg(parser=p)
    add_tag_name_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "shell",
        help="open shell in Python environment",
        description="Open shell in Python environment",
        func=lambda logger, args: do_shell(
            logger=logger,
            cache_dir=args.cache_dir,
            env=args.env))
    add_common_args(p)

    p = add_subcommand(
        subparsers,
        "available",
        help="list available Python versions",
        description="List available Python versions",
        func=lambda ctx, args: do_available(
            ctx=ctx,
            asset_filter=AssetFilter.default(
                tag_name=args.tag_name,
                python_version=args.python_version)))
    add_cache_dir_arg(p)
    add_tag_name_arg(p)
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
    logger.setLevel(logging.INFO)
    args.func(ctx=Context(logger=logger, cache_dir=args.cache_dir), args=args)


if __name__ == "__main__":
    colorama.init()
    try:
        main(cwd=os.getcwd(), argv=sys.argv[1:])
    except ReportableError as e:
        xprint(colorama.Fore.RED, str(e), file=sys.stderr)
        exit(1)
