from isopy_lib.env import env_manifest_path, env_root_dir as __env_root_dir
from isopy_lib.manifest import EnvManifest
import os


def do_list(ctx):
    env_root_dir = __env_root_dir(cache_dir=ctx.cache_dir)
    if os.path.exists(env_root_dir):
        for d in sorted(os.listdir(env_root_dir)):
            p = env_manifest_path(cache_dir=ctx.cache_dir, env=d)
            env_manifest = EnvManifest.load(p)
            print(f"{d}: {env_manifest.tag_name}, {env_manifest.python_version}")
    else:
        print("There are no environments yet!")
