
from collections import namedtuple
from isopy_lib.asset import load_releases
from isopy_lib.fs import make_file_path
from isopy_lib.platform import Platform
from isopy_lib.web import download_file
import os


class AssetFilter(namedtuple("AssetFilter", ["tag_name", "python_version", "os_", "arch", "flavour"])):
    @staticmethod
    def default(tag_name, python_version):
        platform = Platform.current()
        if platform == Platform.LINUX:
            os_ = "linux"
            flavour = "gnu"
        elif platform == Platform.MACOS:
            os_ = "darwin"
            flavour = None
        else:
            raise NotImplementedError(f"Unsupported platform {platform}")

        return AssetFilter(
            tag_name=tag_name,
            python_version=python_version,
            os_=os_,
            arch="x86_64",
            flavour=flavour)

    def __str__(self):
        return ", ".join(
            f"{k}={v}"
            for k, v in self._asdict().items()
            if v is not None)


def make_release_predicate(asset_filter):
    def predicate(x):
        if asset_filter.tag_name is not None:
            if x.tag_name != asset_filter.tag_name:
                return False
        return True
    return predicate


def make_asset_predicate(asset_filter):
    def predicate(x):
        if asset_filter.python_version is not None:
            if x.python_version != asset_filter.python_version:
                return False
        if asset_filter.os_ is not None:
            if x.os != asset_filter.os_:
                return False
        if asset_filter.arch is not None:
            if x.arch != asset_filter.arch:
                return False
        if asset_filter.flavour is not None:
            if x.flavour != asset_filter.flavour:
                return False
        return True
    return predicate


def get_assets(logger, cache_dir, asset_filter):
    def filter_releases(releases):
        return filter(make_release_predicate(asset_filter=asset_filter), releases)

    def filter_assets(assets):
        return filter(make_asset_predicate(asset_filter=asset_filter), assets)

    index_path = make_file_path(
        cache_dir,
        "assets",
        "index.json")
    if os.path.isfile(index_path):
        logger.info(
            f"Found cached releases data at {index_path}")
    else:
        download_file(
            url="https://api.github.com/repos/indygreg/python-build-standalone/releases",
            local_path=index_path)

    releases = load_releases(index_path)

    return sorted([
        asset
        for release in filter_releases(releases=releases)
        for asset in filter_assets(assets=release.assets)
    ], key=lambda x: (x.tag_name, x.python_version), reverse=True)
