from collections import namedtuple
from enum import Enum, auto, unique
from tempfile import TemporaryDirectory
import argparse
import json
import logging
import os
import pty
import requests
import shutil
import sys


PYTHON_FILE_NAME_FORMAT = "cpython-{python_version}+{tag_name}-x86_64-unknown-linux-gnu-install_only.tar.gz"
PYTHON_URL_FORMAT = "https://github.com/indygreg/python-build-standalone/releases/download/{tag_name}/{file_name}"
EXTS = set([".tar.gz"])
ARCHES = set(["aarch64", "x86_64", "x86_64_v2", "x86_64_v3", "x86_64_v4"])
SUBARCHES = set(["apple", "pc", "unknown"])
OSES = set(["darwin", "linux"])
FLAVOURS = set(["debug", "gnu", "musl"])
SUBFLAVOURS = set(["install_only"])


from enum import Enum, auto, unique
import platform


@unique
class Platform(Enum):
    LINUX = auto()
    MACOS = auto()
    WINDOWS = auto()

    @classmethod
    def current(cls):
        os = platform.system().lower()
        if os == "linux":
            return cls.LINUX
        elif os == "darwin":
            return cls.MACOS
        elif os == "windows":
            return cls.WINDOWS
        else:
            raise NotImplementedError(f"Unsupported OS \"{os}\"")


ReleaseInfo = namedtuple("ReleaseInfo", [
    "tag_name",
    "assets"
])


AssetInfo = namedtuple("AssetInfo", [
    "browser_download_url",
    "name",
    "ext",
    "python_version",
    "tag_name",
    "arch",
    "subarch",
    "os",
    "flavour",
    "subflavour"
])


def make_dir_path(*args):
    return os.path.abspath(os.path.join(*args))


def make_file_path(*args):
    return os.path.abspath(os.path.join(*args))


def split_at_ext(s):
    for ext in EXTS:
        if s.endswith(ext):
            s_len = len(s)
            ext_len = len(ext)
            temp = s_len - ext_len
            return s[0:temp], s[temp:]
    raise ValueError(f"Name {s} has unknown extension")


def parse_python_version_and_tag_name(s):
    parts = s.split("+")
    if len(parts) != 2:
        raise ValueError(f"Invalid Python version and tag name {s}")

    return parts


def make_release_info(obj):
    assets = []
    for asset_obj in obj["assets"]:
        try:
            assets.append(make_asset_info(asset_obj))
        except ValueError:
            pass

    return ReleaseInfo(tag_name=obj["tag_name"], assets=assets)


def make_asset_info(obj):
    base, ext = split_at_ext(obj["name"])

    prog, temp, arch, subarch, os_, flavour, subflavour = \
        base.split("-", 7)

    if prog != "cpython":
        raise ValueError(f"Invalid program name {prog}")

    python_version, tag_name = parse_python_version_and_tag_name(temp)

    if arch not in ARCHES:
        raise ValueError(f"Unsupported architecture {arch}")

    if subarch not in SUBARCHES:
        raise ValueError(f"Unsupported subarchitecture {subarch}")

    if os_ not in OSES:
        raise ValueError(f"Unsupported OS {os_}")

    if flavour not in FLAVOURS:
        raise ValueError(f"Unsupported flavour {flavour}")

    if subflavour not in SUBFLAVOURS:
        raise ValueError(f"Unsupported subflavour {subflavour}")

    return AssetInfo(
        browser_download_url=obj["browser_download_url"],
        name=obj["name"],
        ext=ext,
        python_version=python_version,
        tag_name=tag_name,
        arch=arch,
        subarch=subarch,
        os=os_,
        flavour=flavour,
        subflavour=subflavour)


def download_file(url, local_path):
    local_dir = os.path.dirname(local_path)
    os.makedirs(local_dir, exist_ok=True)
    response = requests.get(url, allow_redirects=True)
    with open(local_path, "wb") as f:
        f.write(response.content)


def make_env_dir(cache_dir, env):
    return make_dir_path(cache_dir, "env", env)


def make_env_manifest_path(cache_dir, env):
    return make_file_path(make_env_dir(cache_dir, env), "env.json")


def do_install(logger, cache_dir, tag_name, python_version, env):
    python_file_name = PYTHON_FILE_NAME_FORMAT.format(
        tag_name=tag_name,
        python_version=python_version)
    python_path = make_file_path(cache_dir, python_file_name)
    if os.path.isfile(python_path):
        logger.info(f"Using {python_path}")
    else:
        python_url = PYTHON_URL_FORMAT.format(
            tag_name=tag_name,
            python_version=python_version,
            file_name=python_file_name)

        logger.info(f"Downloading {python_url} to {python_path}")

        download_file(
            url=python_url,
            local_path=python_path)

    env_dir = make_env_dir(cache_dir=cache_dir, env=env)

    python_dir = make_dir_path(
        env_dir,
        f"cpython-{python_version}+{tag_name}")
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
        with open(env_manifest_path, "wt") as f:
            f.write(json.dumps({
                "tag_name": tag_name,
                "python_version": python_version,
                "python_dir": os.path.relpath(python_dir, env_dir)
            }, indent=2))


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
    existing_path = os.getenv("PATH")
    os.environ["PATH"] = python_bin_dir \
        if existing_path is None \
        else python_bin_dir + ":" + existing_path
    pty.spawn(os.getenv("SHELL"))
    print("You are back in the parent shell")


def do_versions(logger, cache_dir, tag_name=None, python_version=None, os_=None, arch=None, flavour=None):
    def filter_releases(releases, tag_name):
        def predicate(x):
            if tag_name is not None:
                if x.tag_name != tag_name:
                    return False
            return True

        return filter(predicate, releases)

    def filter_assets(assets, python_version, os_, arch, flavour):
        def predicate(x):
            if python_version is not None:
                if x.python_version != python_version:
                    return False
            if os_ is not None:
                if x.os != os_:
                    return False
            if arch is not None:
                if x.arch != arch:
                    return False
            if flavour is not None:
                if x.flavour != flavour:
                    return False
            return True

        return filter(predicate, assets)

    cached_releases_json_path = make_file_path(
        cache_dir,
        "cached-releases.json")
    if os.path.isfile(cached_releases_json_path):
        logger.info(
            f"Found cached releases data at {cached_releases_json_path}")
    else:
        response = requests.get(
            "https://api.github.com/repos/indygreg/python-build-standalone/releases")
        response.raise_for_status()
        with open(cached_releases_json_path, "wt") as f:
            f.write(json.dumps(response.json(), indent=2))

    with open(cached_releases_json_path, "rt") as f:
        releases_obj = json.load(f)

    temp = map(make_release_info, releases_obj)
    releases = sorted(temp, key=lambda x: x.tag_name, reverse=True)

    for release in filter_releases(
            releases=releases,
            tag_name=tag_name):
        for asset in filter_assets(
                assets=release.assets,
                python_version=python_version,
                os_="linux",
                arch="x86_64",
                flavour="gnu"):
            print(asset)


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

    def add_common_args(parser):
        add_cache_dir_arg(parser)
        add_env_dir_arg(parser)

    parser = argparse.ArgumentParser(
        prog="isopy",
        description="Isolated Python Tool")

    subparsers = parser.add_subparsers(required=True)

    p = add_subcommand(
        subparsers,
        "install",
        help="install Python interpreter",
        description="Install Python interpreter",
        func=lambda logger, args: do_install(
            logger=logger,
            cache_dir=args.cache_dir,
            tag_name=args.tag_name,
            python_version=args.python_version,
            env=args.env))
    add_common_args(p)
    p.add_argument(
        "tag_name",
        metavar="TAG_NAME",
        type=str,
        help="tag name")
    p.add_argument(
        "python_version",
        metavar="PYTHON_VERSION",
        type=str,
        help="Python version")

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
        "versions",
        help="list available Python versions",
        description="List available Python versions",
        func=lambda logger, args: do_versions(
            logger=logger,
            cache_dir=args.cache_dir,
            tag_name=args.tag_name,
            python_version=args.python_version))
    add_cache_dir_arg(p)
    p.add_argument(
        "--tag-name",
        "-t",
        metavar="TAG_NAME",
        type=str,
        required=False,
        help="tag name")
    p.add_argument(
        "--python-version",
        "-v",
        metavar="PYTHON_VERSION",
        type=str,
        required=False,
        help="Python version")

    args = parser.parse_args(argv)

    logging.basicConfig(
        format="%(asctime)s %(levelname)s %(message)s")
    logger = logging.getLogger(__name__)
    logger.setLevel(logging.INFO)
    args.func(logger=logger, args=args)


if __name__ == "__main__":
    main(cwd=os.getcwd(), argv=sys.argv[1:])
