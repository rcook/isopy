from isopy_lib.env import env_dir as __env_dir, env_manifest_path
from isopy_lib.fs import dir_path
from isopy_lib.platform import Platform
import json
import os


def do_shell(ctx, env):
    if Platform.current() not in [Platform.LINUX, Platform.MACOS]:
        raise NotImplementedError(f"Not supported for this platform yet")

    with open(env_manifest_path(cache_dir=ctx.cache_dir, env=env), "rt") as f:
        manifest = json.load(f)

    python_dir = dir_path(
        __env_dir(cache_dir=ctx.cache_dir, env=env),
        manifest["python_dir"])
    python_bin_dir = dir_path(python_dir, "bin")

    print(f"Python shell for environment {env}; Python is at {python_bin_dir}")
    print(f"Type \"exit\" to return to parent shell")

    e = dict(os.environ)
    temp = e.get("PATH")
    paths = [] if temp is None else temp.split(":")
    if python_bin_dir not in paths:
        e["PATH"] = ":".join([python_bin_dir] + paths)

    shell = os.getenv("SHELL")
    os.execle(shell, shell, e)
