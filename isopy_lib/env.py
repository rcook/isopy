from collections import namedtuple
from contextlib import contextmanager
from isopy_lib.errors import ReportableError
from isopy_lib.fs import dir_path, file_path
from isopy_lib.platform import Platform
from isopy_lib.version import Version
import os
import yaml


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


def read_yaml(path):
    with open(path, "rt") as f:
        return yaml.load(f, Loader=yaml.SafeLoader)


def write_yaml(path, obj):
    with open(path, "wt") as f:
        yaml.dump(obj, f)


class EnvManifest(namedtuple("EnvManifest", ["tag_name", "python_version", "python_dir"])):
    @staticmethod
    def load(path):
        obj = read_yaml(path)
        tag_name = obj["tag_name"]
        python_version = Version.parse(obj["python_version"])
        python_dir = obj["python_dir"]
        return EnvManifest(
            tag_name=tag_name,
            python_version=python_version,
            python_dir=python_dir)

    def save(self, path):
        write_yaml(path, {
            "tag_name": self.tag_name,
            "python_version": str(self.python_version),
            "python_dir": self.python_dir
        })


class ProjectManifest(namedtuple("ProjectManifest", ["tag_name", "python_version"])):
    FILE_NAME = ".isopy.yaml"

    @staticmethod
    def load_from_dir(dir):
        p = file_path(dir, ProjectManifest.FILE_NAME)
        obj = read_yaml(p)
        tag_name = obj["tag_name"]
        python_version = Version.parse(obj["python_version"])
        return ProjectManifest(
            tag_name=tag_name,
            python_version=python_version)

    def save_to_dir(self, dir, force):
        p = file_path(dir, ProjectManifest.FILE_NAME)
        if not force and os.path.exists(p):
            raise ReportableError(
                f"Project manifest already found at {p}; pass --force to overwrite")

        write_yaml(p, {
            "tag_name": self.tag_name,
            "python_version": str(self.python_version)
        })


class LocalProjectManifest(namedtuple("LocalProjectManifest", ["env"])):
    FILE_NAME = ".isopy.local.yaml"

    @staticmethod
    def load_from_dir(dir):
        p = file_path(dir, LocalProjectManifest.FILE_NAME)
        obj = read_yaml(p)
        env = obj["env"]
        return LocalProjectManifest(env=env)

    def save_to_dir(self, dir, force):
        p = file_path(dir, LocalProjectManifest.FILE_NAME)
        if not force and os.path.exists(p):
            raise ReportableError(
                f"Local project manifest already found at {p}; pass --force to overwrite")

        write_yaml(p, {"env": self.env})


class EnvInfo(namedtuple("EnvInfo", ["env", "tag_name", "python_version", "dir"])):
    @staticmethod
    def load_all(cache_dir):
        dir = env_root_dir(cache_dir=cache_dir)
        return [
            x for x in [
                EnvInfo.load(cache_dir=cache_dir, env=d)
                for d in sorted(os.listdir(dir))
            ]
            if x is not None
        ]

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
