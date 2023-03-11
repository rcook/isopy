from isopy_lib.asset import assets_dir, get_asset
from isopy_lib.env import EnvInfo, EnvManifest, env_dir as __env_dir, env_manifest_path as __env_manifest_path
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path, file_path
from tempfile import TemporaryDirectory
import shutil
import os


def do_new(ctx, env, asset_filter):
    env_infos = EnvInfo.load_all(cache_dir=ctx.cache_dir)
    for e in env_infos:
        if e.env == env:
            raise ReportableError(f"Environment {env} already exists")

    asset = get_asset(ctx=ctx, asset_filter=asset_filter)

    python_path = file_path(assets_dir(ctx.cache_dir), asset.name)
    env_dir = __env_dir(cache_dir=ctx.cache_dir, env=env)
    python_dir = dir_path(
        env_dir,
        f"cpython-{asset.python_version}+{asset.tag_name}")
    if os.path.isdir(python_dir):
        ctx.logger.info(f"Python already exists at {python_dir}")
    else:
        if not os.path.exists(python_path):
            ctx.logger.debug(
                f"Downloading {asset.browser_download_url} to {python_path}")
            asset.download(python_path)

        ctx.logger.debug(f"Unpacking {python_path} to {python_dir}")
        with TemporaryDirectory() as d:
            shutil.unpack_archive(python_path, d)
            temp_python_dir = dir_path(d, "python")
            shutil.move(temp_python_dir, python_dir)

    env_manifest_path = __env_manifest_path(cache_dir=ctx.cache_dir, env=env)
    if os.path.isfile(env_manifest_path):
        ctx.logger.debug(
            f"Environment manifest {env_manifest_path} already exists")
    else:
        EnvManifest(
            python_version=asset.python_version,
            tag_name=asset.tag_name,
            python_dir=os.path.relpath(python_dir, env_dir)).save(env_manifest_path)
