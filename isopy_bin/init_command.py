from isopy_bin.new_command import do_new
from isopy_lib.env import EnvInfo, LocalProjectManifest, ProjectManifest
from isopy_lib.errors import ReportableError


def write_project_manifests(ctx, tag_name, python_version, env, force):
    m0 = ProjectManifest(
        tag_name=tag_name,
        python_version=python_version)
    ctx.logger.info(f"Creating project manifest at {ctx.cwd}")
    m0.save_to_dir(ctx.cwd, force=force)

    m1 = LocalProjectManifest(env=env)
    ctx.logger.info(
        f"Creating local project manifest at {ctx.cwd}")
    m1.save_to_dir(ctx.cwd, force=force)


def do_init(ctx, env, asset_filter, force):
    env_infos = EnvInfo.load_all(cache_dir=ctx.cache_dir)
    for e in env_infos:
        if e.env == env:
            raise ReportableError(f"Environment {env} already exists")

    write_project_manifests(
        ctx=ctx,
        tag_name=asset_filter.tag_name,
        python_version=asset_filter.python_version,
        env=env,
        force=force)
    do_new(ctx=ctx, env=env, asset_filter=asset_filter)
