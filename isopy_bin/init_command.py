from isopy_lib.asset import AssetFilter, get_asset
from isopy_lib.env import EnvInfo, LocalProjectManifest, ProjectManifest
from isopy_lib.errors import ReportableError


def do_init(ctx, env, force):
    env_infos = EnvInfo.load_all(cache_dir=ctx.cache_dir)
    for e in env_infos:
        if e.env == env:
            raise ReportableError(f"Environment {env} already exists")

    project_manifest = ProjectManifest.load_from_dir(dir=ctx.cwd)

    asset_filter = AssetFilter.default(
        tag_name=project_manifest.tag_name,
        python_version=project_manifest.python_version)

    asset = get_asset(ctx=ctx, asset_filter=asset_filter)
    asset.download(ctx=ctx)
    asset.unpack(ctx=ctx, env=env)
    local_project_manifest = LocalProjectManifest(env=env)
    ctx.logger.info(f"Creating local project manifest in directory {ctx.cwd}")
    local_project_manifest.save_to_dir(dir=ctx.cwd, force=force)
