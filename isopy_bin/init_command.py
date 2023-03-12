from hashlib import md5
from isopy_lib.asset import AssetFilter, get_asset
from isopy_lib.env import DirConfig, EnvConfig
from isopy_lib.errors import ReportableError


def do_init(ctx):
    dir_config = DirConfig.find(ctx=ctx)
    if dir_config is None:
        raise ReportableError(
            f"No isopy configuration found for directory {ctx.cwd}; "
            "consider creating one with \"isopy new\"")

    env_config = EnvConfig.find(ctx=ctx, dir_config_path=dir_config.path)
    if env_config is not None:
        raise ReportableError(
            f"Environment already exists for {dir_config.path}")

    asset_filter = AssetFilter.default(
        tag_name=dir_config.tag_name,
        python_version=dir_config.python_version)
    asset = get_asset(ctx=ctx, asset_filter=asset_filter)
    EnvConfig.create(
        ctx=ctx,
        dir_config=dir_config,
        asset=asset)
