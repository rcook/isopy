from collections import namedtuple
from isopy_lib.checksum import make_checksum_file_path, verify_checksum
from isopy_lib.errors import ReportableError
from isopy_lib.fs import file_path, move_file, named_temporary_file, split_at_ext
from isopy_lib.platform import Platform
from isopy_lib.utils import parse_python_version_and_tag_name
from isopy_lib.web import download_file
import os
import json


PYTHON_INDEX_URL = "https://api.github.com/repos/indygreg/python-build-standalone/releases"


EXTS = set([".tar.gz"])
ARCHES = set(["aarch64", "x86_64", "x86_64_v2", "x86_64_v3", "x86_64_v4"])
SUBARCHES = set(["apple", "pc", "unknown"])
OSES = set(["darwin", "linux"])
FLAVOURS = set(["debug", "gnu", "musl"])
SUBFLAVOURS = set(["install_only"])


class ReleaseInfo(namedtuple("ReleaseInfo", ["tag_name", "assets"])):
    @staticmethod
    def load_all(json_path):
        with open(json_path, "rt") as f:
            releases_obj = json.load(f)

        return map(ReleaseInfo.read, releases_obj)

    @staticmethod
    def read(obj):
        assets = []
        for asset_obj in obj["assets"]:
            try:
                assets.append(AssetInfo.read(asset_obj))
            except ValueError:
                pass

        return ReleaseInfo(tag_name=obj["tag_name"], assets=assets)


class AssetInfo(namedtuple("AssetInfo", ["browser_download_url", "name", "ext", "python_version", "tag_name", "arch", "subarch", "os", "flavour", "subflavour"])):
    @staticmethod
    def read(obj):
        asset_name = obj["name"]
        base, ext = split_at_ext(asset_name, EXTS)
        tail = base.split("-")

        prog, *tail = tail
        if prog != "cpython":
            raise ValueError(f"Invalid program name {prog}")

        temp, *tail = tail
        python_version, tag_name = parse_python_version_and_tag_name(temp)

        arch, *tail = tail
        if arch not in ARCHES:
            raise ValueError(f"Unsupported architecture {arch}")

        subarch, *tail = tail
        if subarch not in SUBARCHES:
            raise ValueError(f"Unsupported subarchitecture {subarch}")

        os_, *tail = tail
        if os_ not in OSES:
            raise ValueError(f"Unsupported OS {os_}")

        if os_ == "darwin":
            subflavour, *tail = tail
            if subflavour not in SUBFLAVOURS:
                raise ValueError(f"Unsupported subflavour {subflavour}")

            if tail != []:
                raise ValueError(f"Unsupported asset name {asset_name}")

            return AssetInfo(
                browser_download_url=obj["browser_download_url"],
                name=obj["name"],
                ext=ext,
                python_version=python_version,
                tag_name=tag_name,
                arch=arch,
                subarch=subarch,
                os=os_,
                flavour=None,
                subflavour=subflavour)
        elif os_ == "linux":
            flavour, *tail = tail
            if flavour not in FLAVOURS:
                raise ValueError(f"Unsupported flavour {flavour}")

            subflavour, *tail = tail
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
        else:
            raise NotImplementedError(f"Unsupported OS {os_}")

    def download(self, path):
        with named_temporary_file() as f:
            download_file(
                url=self.browser_download_url,
                local_path=f.name)

            checksum_file_path = make_checksum_file_path(
                tag_name=self.tag_name)
            if not verify_checksum(
                    file_path=f.name,
                    checksum_file_path=checksum_file_path,
                    file_name_key=self.name):
                os.remove(f.name)
                raise ReportableError(
                    f"Checksum verification on downloaded file {f.name} failed; file deleted")

            move_file(f.name, path)


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


def release_predicate(asset_filter):
    def predicate(x):
        if asset_filter.tag_name is not None:
            if x.tag_name != asset_filter.tag_name:
                return False
        return True
    return predicate


def asset_predicate(asset_filter):
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


def get_assets(ctx, asset_filter):
    def filter_releases(releases):
        return filter(release_predicate(asset_filter=asset_filter), releases)

    def filter_assets(assets):
        return filter(asset_predicate(asset_filter=asset_filter), assets)

    index_path = file_path(
        ctx.cache_dir,
        "assets",
        "index.json")
    if os.path.isfile(index_path):
        ctx.logger.debug(
            f"Found Python version index at {index_path}")
    else:
        download_file(
            url=PYTHON_INDEX_URL,
            local_path=index_path)

    releases = ReleaseInfo.load_all(index_path)

    return sorted([
        asset
        for release in filter_releases(releases=releases)
        for asset in filter_assets(assets=release.assets)
    ], key=lambda x: (x.tag_name, x.python_version), reverse=True)


def get_asset(ctx, asset_filter):
    assets = get_assets(ctx=ctx, asset_filter=asset_filter)

    asset_count = len(assets)
    if asset_count == 0:
        raise ReportableError(
            f"There are no Python distributions matching filter {asset_filter}")

    asset = assets[0]
    return asset
