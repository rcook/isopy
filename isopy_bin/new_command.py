from isopy_lib.env import ProjectManifest


def do_new(ctx, asset_filter, force):
    project_manifest = ProjectManifest(
        tag_name=asset_filter.tag_name,
        python_version=asset_filter.python_version)
    ctx.logger.info(f"Creating project manifest in directory {ctx.cwd}")
    project_manifest.save_to_dir(dir=ctx.cwd, force=force)
