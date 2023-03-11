from contextlib import contextmanager
from isopy_lib.fs import dir_path, file_path
from isopy_lib.manifest import EnvManifest
from isopy_lib.platform import Platform
import json
import os


def env_root_dir(cache_dir):
    return dir_path(cache_dir, "env")


def env_dir(cache_dir, env):
    return dir_path(env_root_dir(cache_dir=cache_dir), env)


def env_manifest_path(cache_dir, env):
    return file_path(env_dir(cache_dir, env), "env.json")


@contextmanager
def exec_environment(ctx, env):
    if Platform.current() not in [Platform.LINUX, Platform.MACOS]:
        raise NotImplementedError(f"Not supported for this platform yet")

    manifest = EnvManifest.load(
        env_manifest_path(
            cache_dir=ctx.cache_dir,
            env=env))

    python_dir = dir_path(
        env_dir(cache_dir=ctx.cache_dir, env=env),
        manifest.python_dir)
    python_bin_dir = dir_path(python_dir, "bin")

    e = dict(os.environ)
    temp = e.get("PATH")
    paths = [] if temp is None else temp.split(":")
    if python_bin_dir not in paths:
        e["PATH"] = ":".join([python_bin_dir] + paths)

    yield python_bin_dir, e
