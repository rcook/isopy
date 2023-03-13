from isopy_lib.env import EnvConfig
from isopy_lib.errors import ReportableError


def do_use(ctx, env):
    env_config = EnvConfig.find(ctx=ctx, name=env)
    if env_config is None:
        raise ReportableError(
            f"No environment available named {env}")

    raise NotImplementedError()
