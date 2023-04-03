from isopy_lib.asset import get_asset
from isopy_lib.env import EnvConfig
from isopy_lib.errors import ReportableError


def do_create(ctx, env, asset_filter):
    env_config = EnvConfig.find(ctx=ctx, name=env)
    if env_config is not None:
        raise ReportableError(
            f"Environment already exists with name {env}")

    asset = get_asset(ctx=ctx, asset_filter=asset_filter)
    c = EnvConfig.create(
        ctx=ctx,
        name=env,
        asset=asset)
    ctx.logger.info(f"Initialized environment with name {c.name}")
