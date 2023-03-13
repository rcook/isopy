from collections import namedtuple
from hashlib import md5
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path, file_path
from isopy_lib.platform import Platform
from isopy_lib.version import Version
from isopy_lib.yaml_utils import read_yaml, write_yaml
import os


DIR_CONFIG_FILE_NAME = ".isopy.yaml"
ENV_CONFIG_FILE = "env.json"


class DirConfig(namedtuple("DirConfig", ["path", "tag_name", "python_version"])):
    @staticmethod
    def find(ctx):
        def find_dir_config_path(dir, limit=3):
            if limit == 0:
                return None

            p = file_path(dir, DIR_CONFIG_FILE_NAME)
            if os.path.isfile(p):
                return p

            parent_dir = os.path.dirname(dir)
            if parent_dir == dir:
                return None

            return find_dir_config_path(dir=parent_dir, limit=limit - 1)

        p = find_dir_config_path(dir=ctx.cwd)
        if p is None:
            return None

        return DirConfig._from_obj(path=p, obj=read_yaml(p))

    @staticmethod
    def create(ctx, tag_name, python_version):
        p = file_path(ctx.cwd, DIR_CONFIG_FILE_NAME)
        c = DirConfig(
            path=p,
            tag_name=tag_name,
            python_version=python_version)
        write_yaml(p, {
            "tag_name": str(c.tag_name),
            "python_version": str(c.python_version)
        })
        return c

    @staticmethod
    def _from_obj(path, obj):
        tag_name = obj["tag_name"]
        python_version = Version.parse(obj["python_version"])
        return DirConfig(
            path=path,
            tag_name=tag_name,
            python_version=python_version)


class EnvConfig(namedtuple("EnvConfig", ["path", "name", "dir_config_path", "tag_name", "python_version", "python_dir"])):
    @staticmethod
    def load_by_name(ctx, env):
        envs_dir = dir_path(ctx.cache_dir, "envs")
        env_config_path = file_path(envs_dir, env, ENV_CONFIG_FILE)
        return EnvConfig._from_obj(
            ctx=ctx,
            path=env_config_path,
            obj=read_yaml(env_config_path))

    @staticmethod
    def load_all(ctx):
        def get_env_configs(dir_name):
            dir = dir_path(ctx.cache_dir, dir_name)
            if os.path.isdir(dir):
                fs = [
                    file_path(dir, d, ENV_CONFIG_FILE)
                    for d in os.listdir(dir)
                ]
                return [
                    EnvConfig._from_obj(ctx=ctx, path=f, obj=read_yaml(f))
                    for f in fs
                    if os.path.isfile(f)
                ]
            else:
                return []

        return get_env_configs("envs") + get_env_configs("hashed")

    @staticmethod
    def find(ctx, dir_config_path):
        env_dir = EnvConfig._dir(ctx=ctx, dir_config_path=dir_config_path)
        env_config_path = file_path(env_dir, ENV_CONFIG_FILE)

        try:
            obj = read_yaml(env_config_path)
        except FileNotFoundError:
            return None

        return EnvConfig._from_obj(
            ctx=ctx,
            path=env_config_path,
            obj=obj)

    @staticmethod
    def create(ctx, dir_config, asset):
        env_dir = EnvConfig._dir(ctx=ctx, dir_config_path=dir_config.path)
        env_config_path = file_path(env_dir, ENV_CONFIG_FILE)
        output_dir = asset.extract(ctx=ctx, dir=env_dir)
        python_dir = os.path.relpath(output_dir, env_dir)
        c = EnvConfig(
            path=env_config_path,
            name=None,
            dir_config_path=dir_config.path,
            tag_name=dir_config.tag_name,
            python_version=dir_config.python_version,
            python_dir=python_dir)
        write_yaml(env_config_path, {
            "dir_config_path": dir_config.path,
            "tag_name": str(c.tag_name),
            "python_version": str(dir_config.python_version),
            "python_dir": python_dir
        })
        return c

    @staticmethod
    def _dir(ctx, name=None, dir_config_path=None):
        assert name is None and dir_config_path is not None or \
            name is not None and dir_config_path is None
        if name is None:
            hash = md5(dir_config_path.encode("utf-8")).hexdigest()
            return file_path(ctx.cache_dir, "hashed", hash)
        else:
            return file_path(ctx.cache_dir, "envs", name)

    @staticmethod
    def _from_obj(ctx, path, obj):
        name = obj.get("name", None)
        dir_config_path = obj.get("dir_config_path", None)
        tag_name = obj["tag_name"]
        python_version = Version.parse(obj["python_version"])
        python_dir = obj["python_dir"]
        return EnvConfig(
            path=path,
            name=name,
            dir_config_path=dir_config_path,
            tag_name=tag_name,
            python_version=python_version,
            python_dir=python_dir)

    def make_python_dir(self, ctx):
        return dir_path(
            EnvConfig._dir(
                ctx=ctx,
                name=self.name,
                dir_config_path=self.dir_config_path),
            self.python_dir)


def get_env_config(ctx, env):
    if env is None:
        dir_config = DirConfig.find(ctx=ctx)
        if dir_config is None:
            raise ReportableError(
                f"No isopy configuration found for directory {ctx.cwd}; "
                "consider creating one with \"isopy new\"")

        env_config = EnvConfig.find(ctx=ctx, dir_config_path=dir_config.path)
        if env_config is None:
            raise ReportableError(
                f"No environment initialized for {dir_config.path}")

        return env_config
    else:
        return EnvConfig.load_by_name(ctx=ctx, env=env)
