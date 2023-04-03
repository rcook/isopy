from isopy_lib.env import DirConfig
from isopy_lib.errors import ReportableError


def do_new(ctx, asset_filter):
    try:
        c = DirConfig.create(
            ctx=ctx,
            tag=asset_filter.tag,
            python_version=asset_filter.python_version)
    except FileExistsError as e:
        raise ReportableError(
            f"Project configuration file {e.filename} already exists") from e
    ctx.logger.info(f"Created project configuration at {c.path}")
