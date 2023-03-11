from collections import namedtuple
from isopy_lib.env import env_manifest_path, env_root_dir as __env_root_dir
from isopy_lib.manifest import EnvManifest
from isopy_lib.pretty import show_table
import os


class EnvInfo(namedtuple("EnvInfo", ["env", "tag_name", "python_version", "dir"])):
    @staticmethod
    def load(cache_dir, env):
        p = env_manifest_path(cache_dir=cache_dir, env=env)
        if not os.path.isfile(p):
            return None

        env_manifest = EnvManifest.load(p)
        return EnvInfo(
            env=env,
            tag_name=env_manifest.tag_name,
            python_version=env_manifest.python_version,
            dir=p)


def do_list(ctx):
    env_root_dir = __env_root_dir(cache_dir=ctx.cache_dir)
    envs = [
        x for x in [
            EnvInfo.load(cache_dir=ctx.cache_dir, env=d)
            for d in sorted(os.listdir(env_root_dir))
        ]
        if x is not None
    ]

    if len(envs) > 0:
        show_table(items=envs)
    else:
        print("There are no environments yet!")
