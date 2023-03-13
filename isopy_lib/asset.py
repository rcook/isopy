from collections import namedtuple
from isopy_lib.checksum import make_checksum_file_path, verify_checksum
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path, file_path, move_file, named_temporary_file, split_at_ext
from isopy_lib.platform import PLATFORM
from isopy_lib.utils import parse_python_version_and_tag
from isopy_lib.web import download_file
from tempfile import TemporaryDirectory
import os
import json
import shutil


PYTHON_INDEX_URL = "https://api.github.com/repos/indygreg/python-build-standalone/releases"


EXTS = set([".tar.gz"])
ARCHES = set(["aarch64", "x86_64", "x86_64_v2", "x86_64_v3", "x86_64_v4"])
SUBARCHES = set(["apple", "pc", "unknown"])
OSES = set(["darwin", "linux", "windows"])
FLAVOURS = set(["debug", "gnu", "msvc", "musl"])
SUBFLAVOURS = set(["install_only"])
LINKAGES = set(["shared", "static"])


class ReleaseInfo(namedtuple("ReleaseInfo", ["tag", "assets"])):
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

        return ReleaseInfo(tag=obj["tag_name"], assets=assets)


class AssetNameInfo(namedtuple("AssetNameInfo", ["ext", "python_version", "tag", "tail"])):
    @staticmethod
    def parse(asset_name):
        base, ext = split_at_ext(asset_name, EXTS)
        tail = base.split("-")

        prog, *tail = tail
        if prog != "cpython":
            raise ValueError(f"Invalid program name {prog}")

        temp, *tail = tail
        python_version, tag = parse_python_version_and_tag(temp)
        return AssetNameInfo(
            ext=ext,
            python_version=python_version,
            tag=tag,
            tail=tail)


class AssetInfo(namedtuple("AssetInfo", ["browser_download_url", "name", "ext", "python_version", "tag", "arch", "subarch", "os", "flavour", "subflavour"])):
    @staticmethod
    def read(obj):
        asset_name = obj["name"]

        asset_name_info = AssetNameInfo.parse(asset_name=asset_name)
        ext = asset_name_info.ext
        python_version = asset_name_info.python_version
        tag = asset_name_info.tag
        tail = asset_name_info.tail

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
                tag=tag,
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
                tag=tag,
                arch=arch,
                subarch=subarch,
                os=os_,
                flavour=flavour,
                subflavour=subflavour)
        elif os_ == "windows":
            flavour, *tail = tail
            if flavour not in FLAVOURS:
                raise ValueError(f"Unsupported flavour {flavour}")

            linkage, *tail = tail
            if linkage not in LINKAGES:
                raise ValueError(f"Unsupported linkage {linkage}")

            subflavour, *tail = tail
            if subflavour not in SUBFLAVOURS:
                raise ValueError(f"Unsupported subflavour {subflavour}")

            return AssetInfo(
                browser_download_url=obj["browser_download_url"],
                name=obj["name"],
                ext=ext,
                python_version=python_version,
                tag=tag,
                arch=arch,
                subarch=subarch,
                os=os_,
                flavour=flavour,
                subflavour=subflavour)
        else:
            raise NotImplementedError(f"Unsupported OS {os_}")

    def download(self, ctx):
        p = self._path(ctx=ctx)
        if os.path.exists(p):
            ctx.logger.info(f"Asset is already downloaded at {p}")
            return

        with named_temporary_file() as f:
            download_file(
                url=self.browser_download_url,
                local_path=f.name)

            checksum_file_path = make_checksum_file_path(tag=self.tag)
            if not verify_checksum(
                    file_path=f.name,
                    checksum_file_path=checksum_file_path,
                    file_name_key=self.name):
                os.remove(f.name)
                raise ReportableError(
                    f"Checksum verification on downloaded asset {f.name} failed; "
                    "file deleted")

            move_file(f.name, p)
            ctx.logger.info(f"Asset downloaded to {p}")

    def extract(self, ctx, dir):
        self.download(ctx=ctx)

        output_dir = dir_path(
            dir,
            f"cpython-{self.python_version}+{self.tag}")
        if os.path.isdir(output_dir):
            ctx.logger.info(f"Asset already extracted at {output_dir}")
            return output_dir

        p = self._path(ctx=ctx)
        ctx.logger.debug(f"Unpacking {p} to {output_dir}")
        with TemporaryDirectory() as d:
            shutil.unpack_archive(p, d)
            shutil.move(dir_path(d, "python"), output_dir)

        return output_dir

    def _path(self, ctx):
        return file_path(assets_dir(ctx.cache_dir), self.name)


class AssetFilter(namedtuple("AssetFilter", ["tag", "python_version", "os_", "arch", "flavour"])):
    @staticmethod
    def default(tag, python_version):
        return AssetFilter(
            tag=tag,
            python_version=python_version,
            os_=PLATFORM.asset_os,
            arch="x86_64",
            flavour=PLATFORM.asset_flavour)

    def __str__(self):
        return ", ".join(
            f"{k}={v}"
            for k, v in self._asdict().items()
            if v is not None)


def assets_dir(cache_dir):
    return dir_path(cache_dir, "assets")


def release_predicate(asset_filter):
    def predicate(x):
        if asset_filter.tag is not None:
            if x.tag != asset_filter.tag:
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


def get_assets(ctx, asset_filter, refresh):
    def filter_releases(releases):
        return filter(release_predicate(asset_filter=asset_filter), releases)

    def filter_assets(assets):
        return filter(asset_predicate(asset_filter=asset_filter), assets)

    index_path = file_path(assets_dir(ctx.cache_dir), "index.json")
    if os.path.isfile(index_path):
        if refresh:
            with named_temporary_file() as f:
                download_file(
                    url=PYTHON_INDEX_URL,
                    local_path=f.name)
                move_file(f.name, index_path, overwrite=True)
            ctx.logger.debug(
                f"Updated Python version index at {index_path}")
        else:
            ctx.logger.debug(
                f"Found Python version index at {index_path}")
    else:
        download_file(
            url=PYTHON_INDEX_URL,
            local_path=index_path)
        ctx.logger.debug(
            f"Downloaded Python version index to {index_path}")

    releases = ReleaseInfo.load_all(index_path)

    return sorted([
        asset
        for release in filter_releases(releases=releases)
        for asset in filter_assets(assets=release.assets)
    ], key=lambda x: (x.tag, x.python_version), reverse=True)


def get_asset(ctx, asset_filter):
    assets = get_assets(ctx=ctx, asset_filter=asset_filter, refresh=False)

    asset_count = len(assets)
    if asset_count == 0:
        raise ReportableError(
            f"There are no Python distributions matching filter {asset_filter}")

    asset = assets[0]
    return asset
