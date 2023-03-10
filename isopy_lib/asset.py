from collections import namedtuple
from isopy_lib.fs import split_at_ext
from isopy_lib.utils import parse_python_version_and_tag_name
import json


EXTS = set([".tar.gz"])
ARCHES = set(["aarch64", "x86_64", "x86_64_v2", "x86_64_v3", "x86_64_v4"])
SUBARCHES = set(["apple", "pc", "unknown"])
OSES = set(["darwin", "linux"])
FLAVOURS = set(["debug", "gnu", "musl"])
SUBFLAVOURS = set(["install_only"])


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


def make_release_info(obj):
    assets = []
    for asset_obj in obj["assets"]:
        try:
            assets.append(make_asset_info(asset_obj))
        except ValueError:
            pass

    return ReleaseInfo(tag_name=obj["tag_name"], assets=assets)


def make_asset_info(obj):
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


def load_releases(json_path):
    with open(json_path, "rt") as f:
        releases_obj = json.load(f)

    return map(make_release_info, releases_obj)
