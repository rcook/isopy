from isopy_lib.fs import dir_path, file_path


def env_root_dir(cache_dir):
    return dir_path(cache_dir, "env")


def env_dir(cache_dir, env):
    return dir_path(env_root_dir(cache_dir=cache_dir), env)


def env_manifest_path(cache_dir, env):
    return file_path(env_dir(cache_dir, env), "env.json")
