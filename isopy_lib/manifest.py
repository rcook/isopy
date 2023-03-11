from collections import namedtuple
from isopy_lib.errors import ReportableError
from isopy_lib.fs import file_path
from isopy_lib.version import Version
import os
import yaml


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
