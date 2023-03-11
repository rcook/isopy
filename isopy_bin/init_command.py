from isopy_bin.new_command import do_new
from isopy_lib.manifest import LocalProjectManifest, ProjectManifest


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
    write_project_manifests(
        ctx=ctx,
        tag_name=asset_filter.tag_name,
        python_version=asset_filter.python_version,
        env=env,
        force=force)
    do_new(ctx=ctx, env=env, asset_filter=asset_filter)
