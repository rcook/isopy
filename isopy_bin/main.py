from isopy_lib.checksum import verify_checksum
from isopy_lib.env import make_env_dir, make_env_manifest_path, make_env_root_dir
from isopy_lib.errors import ReportableError
from isopy_lib.fs import make_dir_path, make_file_path
from isopy_lib.manifest import EnvManifest
from isopy_lib.platform import Platform
from isopy_lib.version import Version
from isopy_lib.assets import get_assets
from isopy_lib.web import download_file
from tempfile import TemporaryDirectory
import argparse
import json
import logging
import os
import shutil
import sys


PYTHON_URL_FORMAT = "https://github.com/indygreg/python-build-standalone/releases/download/{tag_name}/{file_name}"


def do_list(logger, cache_dir):
    env_root_dir = make_env_root_dir(cache_dir=cache_dir)
    if os.path.exists(env_root_dir):
        for d in sorted(os.listdir(env_root_dir)):
            p = make_env_manifest_path(cache_dir=cache_dir, env=d)
            env_manifest = EnvManifest.load(p)
            print(f"{d}: {env_manifest.tag_name}, {env_manifest.python_version}")
    else:
        print("There are no environments yet!")


def get_asset(logger, cache_dir, env, python_version, tag_name=None, os_=None, arch=None, flavour=None):
    assets = get_assets(
        logger=logger,
        cache_dir=cache_dir,
        tag_name=tag_name,
        python_version=python_version,
        os_=os_,
        arch=arch,
        flavour=flavour)

    asset_count = len(assets)
    if asset_count == 0:
        raise ReportableError(
            f"There are no Python distributions matching version {python_version}")

    asset = assets[0]
    return asset


def do_download(logger, cache_dir, env, python_version, tag_name=None, os_=None, arch=None, flavour=None):
    def make_checksum_file_path(tag_name):
        return make_file_path(
            __file__,
            "..",
            "..",
            "sha256sums",
            f"{tag_name}.sha256sums")

    asset = get_asset(
        logger=logger,
        cache_dir=cache_dir,
        env=env,
        python_version=python_version,
        tag_name=tag_name,
        os_=os_,
        arch=arch,
        flavour=flavour)

    python_path = make_file_path(cache_dir, "assets", asset.name)
    if os.path.isfile(python_path):
        logger.info(f"Package {python_path} is already available locally")
    else:
        python_url = PYTHON_URL_FORMAT.format(
            python_version=asset.python_version,
            tag_name=asset.tag_name,
            file_name=asset.name)

        logger.info(f"Downloading {python_url} to {python_path}")

        download_file(
            url=python_url,
            local_path=python_path)

        if not verify_checksum(
                file_path=python_path,
                checksum_file_path=make_checksum_file_path(tag_name=asset.tag_name)):
            os.remove(python_path)
            raise ReportableError(
                f"Checksum verification on downloaded file {python_path} failed")


def do_new(logger, cache_dir, env, force, python_version, tag_name=None, os_=None, arch=None, flavour=None):
    asset = get_asset(
        logger=logger,
        cache_dir=cache_dir,
        env=env,
        python_version=python_version,
        tag_name=tag_name,
        os_=os_,
        arch=arch,
        flavour=flavour)

    python_path = make_file_path(cache_dir, "assets", asset.name)

    env_dir = make_env_dir(cache_dir=cache_dir, env=env)

    python_dir = make_dir_path(
        env_dir,
        f"cpython-{asset.python_version}+{asset.tag_name}")
    if os.path.isdir(python_dir):
        logger.info(f"Python already exists at {python_dir}")
    else:
        logger.info(f"Unpacking {python_path} to {python_dir}")
        with TemporaryDirectory() as d:
            shutil.unpack_archive(python_path, d)
            temp_python_dir = make_dir_path(d, "python")
            shutil.move(temp_python_dir, python_dir)

    env_manifest_path = make_env_manifest_path(cache_dir=cache_dir, env=env)
    if os.path.isfile(env_manifest_path):
        logger.info(f"Environment manifest {env_manifest_path} already exists")
    else:
        EnvManifest(
            python_version=asset.python_version,
            tag_name=asset.tag_name,
            python_dir=os.path.relpath(python_dir, env_dir)).save(env_manifest_path)


def do_shell(logger, cache_dir, env):
    if Platform.current() not in [Platform.LINUX, Platform.MACOS]:
        raise NotImplementedError(f"Not supported for this platform yet")

    with open(make_env_manifest_path(cache_dir=cache_dir, env=env), "rt") as f:
        manifest = json.load(f)

    python_dir = make_dir_path(
        make_env_dir(cache_dir=cache_dir, env=env),
        manifest["python_dir"])
    python_bin_dir = make_dir_path(python_dir, "bin")

    print(f"Python shell for environment {env}; Python is at {python_bin_dir}")
    print(f"Type \"exit\" to return to parent shell")

    e = dict(os.environ)
    temp = e.get("PATH")
    paths = [] if temp is None else temp.split(":")
    if python_bin_dir not in paths:
        e["PATH"] = ":".join([python_bin_dir] + paths)

    shell = os.getenv("SHELL")
    os.execle(shell, shell, e)


def do_available(logger, cache_dir, tag_name=None, python_version=None, os_=None, arch=None, flavour=None):
    assets = get_assets(
        logger=logger,
        cache_dir=cache_dir,
        tag_name=tag_name,
        python_version=python_version,
        os_=os_,
        arch=arch,
        flavour=flavour)

    for asset in assets:
        print(f"{asset.os} {asset.arch} {asset.tag_name} {asset.python_version}")


def main(cwd, argv):
    default_cache_dir = os.path.expanduser("~/.isopy")
    default_env = "default"

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

    def add_env_dir_arg(parser):
        parser.add_argument(
            "--env",
            "-e",
            metavar="ENV",
            default=default_env,
            help=f"cache directory (default: {default_env})")

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
        add_env_dir_arg(parser)

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
            python_version=args.python_version,
            tag_name=args.tag_name))
    add_common_args(p)
    p.add_argument(
        "python_version",
        metavar="PYTHON_VERSION",
        type=Version.parse,
        help="Python version")
    add_tag_name_arg(p)

    p = add_subcommand(
        subparsers,
        "new",
        help="create new isolated Python environment",
        description="Create new isolated Python environment",
        func=lambda logger, args: do_new(
            logger=logger,
            cache_dir=args.cache_dir,
            env=args.env,
            force=args.force,
            python_version=args.python_version,
            tag_name=args.tag_name))
    add_common_args(p)
    p.add_argument(
        "python_version",
        metavar="PYTHON_VERSION",
        type=Version.parse,
        help="Python version")
    add_tag_name_arg(p)
    p.add_argument(
        "--force",
        "-f",
        metavar="FORCE",
        action=argparse.BooleanOptionalAction,
        help="force overwrite of environment")

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
        func=lambda logger, args: do_available(
            logger=logger,
            cache_dir=args.cache_dir,
            tag_name=args.tag_name,
            python_version=args.python_version))
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

    try:
        args.func(logger=logger, args=args)
    except ReportableError as e:
        print(str(e), file=sys.stderr)
        exit(1)


if __name__ == "__main__":
    main(cwd=os.getcwd(), argv=sys.argv[1:])
