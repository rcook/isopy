from isopy_lib.env import env_dir as __env_dir, env_manifest_path
from isopy_lib.fs import dir_path
import json
import os


def do_exec(ctx, env, command):
    with open(env_manifest_path(cache_dir=ctx.cache_dir, env=env), "rt") as f:
        manifest = json.load(f)

    python_dir = dir_path(
        __env_dir(cache_dir=ctx.cache_dir, env=env),
        manifest["python_dir"])
    python_bin_dir = dir_path(python_dir, "bin")

    e = dict(os.environ)
    temp = e.get("PATH")
    paths = [] if temp is None else temp.split(":")
    if python_bin_dir not in paths:
        e["PATH"] = ":".join([python_bin_dir] + paths)

    os.execlpe(command[0], *command, e)
