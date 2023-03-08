from isopy_lib.fs import make_dir_path, make_file_path


def make_env_dir(cache_dir, env):
    return make_dir_path(cache_dir, "env", env)


def make_env_manifest_path(cache_dir, env):
    return make_file_path(make_env_dir(cache_dir, env), "env.json")
