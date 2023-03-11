from collections import namedtuple
from isopy_lib.env import env_manifest_path, env_root_dir as __env_root_dir
from isopy_lib.fs import dir_path
from isopy_lib.manifest import EnvManifest
from isopy_lib.pretty import show_item_table
import os


Env = namedtuple("Env", [
    "env",
    "tag_name",
    "python_version",
    "dir"
])


def do_list(ctx):
    def make_env(env, env_manifest, dir):
        return Env(
            env=env,
            tag_name=env_manifest.tag_name,
            python_version=env_manifest.python_version,
            dir=dir)

    env_root_dir = __env_root_dir(cache_dir=ctx.cache_dir)
    if os.path.exists(env_root_dir):
        envs = [
            make_env(
                env=d,
                env_manifest=EnvManifest.load(
                    env_manifest_path(
                        cache_dir=ctx.cache_dir,
                        env=d)),
                dir=dir_path(env_root_dir, d))
            for d in sorted(os.listdir(env_root_dir))
        ]
        show_item_table(attrs=[
            "env",
            "tag_name",
            "python_version",
            "dir"
        ], items=envs)
    else:
        print("There are no environments yet!")
