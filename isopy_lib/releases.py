from isopy_lib.fs import make_file_path
from isopy_lib.platform import Platform
import json
import requests


def get_versions(logger, cache_dir, tag_name=None, python_version=None, os_=None, arch=None, flavour=None):
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

    platform = Platform.current()
    if platform == Platform.LINUX:
        os_ = "linux"
        flavour = "gnu"
    elif platform == Platform.MACOS:
        os_ = "darwin"
        flavour = None
    else:
        raise NotImplementedError(f"Unsupported platform {platform}")

    with open(cached_releases_json_path, "rt") as f:
        releases_obj = json.load(f)

    releases = map(make_release_info, releases_obj)

    return sorted([
        asset
        for release in filter_releases(releases=releases, tag_name=tag_name)
        for asset in filter_assets(
            assets=release.assets,
            python_version=python_version,
            os_=os_,
            arch="x86_64",
            flavour=flavour)
    ], key=lambda x: (x.tag_name, x.python_version), reverse=True)
