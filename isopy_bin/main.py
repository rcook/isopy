from collections import namedtuple
from tempfile import TemporaryDirectory
import argparse
import json
import logging
import os
import requests
import shutil
import sys


PYTHON_FILE_NAME_FORMAT = "cpython-{python_version}+{tag_name}-x86_64-unknown-linux-gnu-install_only.tar.gz"
PYTHON_URL_FORMAT = "https://github.com/indygreg/python-build-standalone/releases/download/{tag_name}/{file_name}"


def make_dir_path(*args):
    return os.path.abspath(os.path.join(*args))


def make_file_path(*args):
    return os.path.abspath(os.path.join(*args))


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
    print(python_dir)
    exit(1)
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
                "tagName": tag_name,
                "pythonVersion": python_version,
                "pythonDir": os.path.relpath(python_dir, env_dir)
            }, indent=2))


def do_shell(logger, cache_dir, env):
    with open(make_env_manifest_path(cache_dir=cache_dir, env=env), "rt") as f:
        manifest = json.load(f)

    python_dir = make_dir_path(
        make_env_dir(cache_dir=cache_dir, env=env),
        manifest["pythonDir"])
    python_bin_dir = make_dir_path(python_dir, "bin")
    print(f"export PATH={python_bin_dir}:$PATH")


def do_versions(logger, cache_dir):
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

    ReleaseInfo = namedtuple("ReleaseInfo", [
        "tag_name",
        "prerelease",
        "assets"
    ])

    def make_release_info(obj):
        return ReleaseInfo(
            tag_name=obj["tag_name"],
            prerelease=obj["prerelease"],
            assets=obj["assets"])

    temp = map(make_release_info, releases_obj)
    releases = sorted(temp, key=lambda x: x.tag_name, reverse=True)

    release = releases[0]
    for a in release.assets[0:1]:
        print(json.dumps(a, indent=2))
    exit(1)


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
            tag_name="20230116",
            python_version="3.11.1",
            env=args.env))
    add_common_args(p)

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
            cache_dir=args.cache_dir))
    add_cache_dir_arg(p)

    args = parser.parse_args(argv)

    logging.basicConfig(
        format="%(asctime)s %(levelname)s %(message)s")
    logger = logging.getLogger(__name__)
    logger.setLevel(logging.INFO)
    args.func(logger=logger, args=args)


if __name__ == "__main__":
    main(cwd=os.getcwd(), argv=sys.argv[1:])
