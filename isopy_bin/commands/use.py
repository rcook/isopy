from isopy_lib.env import EnvConfig, UseInfo
from isopy_lib.errors import ReportableError


def do_use(ctx, env):
    use_info = UseInfo.find(ctx)
    if use_info is not None:
        raise ReportableError(
            f"Directory {use_info.dir} already "
            f"uses environment {use_info.env}")

    env_config = EnvConfig.find(ctx=ctx, name=env)
    if env_config is None:
        raise ReportableError(
            f"No environment available named {env}")

    UseInfo.create(ctx=ctx, env=env)
