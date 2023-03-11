from isopy_lib.asset import get_asset
from isopy_lib.env import env_dir as __env_dir, env_manifest_path as __env_manifest_path
from isopy_lib.fs import dir_path, file_path
from isopy_lib.manifest import EnvManifest
from tempfile import TemporaryDirectory
import shutil
import os


def do_new(ctx, env, asset_filter):
    asset = get_asset(ctx=ctx, asset_filter=asset_filter)

    python_path = file_path(ctx.cache_dir, "assets", asset.name)
    env_dir = __env_dir(cache_dir=ctx.cache_dir, env=env)
    python_dir = dir_path(
        env_dir,
        f"cpython-{asset.python_version}+{asset.tag_name}")
    if os.path.isdir(python_dir):
        ctx.logger.info(f"Python already exists at {python_dir}")
    else:
        if not os.path.exists(python_path):
            ctx.logger.info(
                f"Downloading {asset.browser_download_url} to {python_path}")
            asset.download(python_path)

        ctx.logger.info(f"Unpacking {python_path} to {python_dir}")
        with TemporaryDirectory() as d:
            shutil.unpack_archive(python_path, d)
            temp_python_dir = dir_path(d, "python")
            shutil.move(temp_python_dir, python_dir)

    env_manifest_path = __env_manifest_path(cache_dir=ctx.cache_dir, env=env)
    if os.path.isfile(env_manifest_path):
        ctx.logger.info(
            f"Environment manifest {env_manifest_path} already exists")
    else:
        EnvManifest(
            python_version=asset.python_version,
            tag_name=asset.tag_name,
            python_dir=os.path.relpath(python_dir, env_dir)).save(env_manifest_path)