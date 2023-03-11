from collections import namedtuple
from isopy_lib.version import Version
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
        obj = read_yaml.load(path)
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
    @staticmethod
    def load(path):
        obj = read_yaml.load(path)
        tag_name = obj["tag_name"]
        python_version = Version.parse(obj["python_version"])
        return ProjectManifest(
            tag_name=tag_name,
            python_version=python_version)

    def save(self, path):
        write_yaml(path, {
            "tag_name": self.tag_name,
            "python_version": str(self.python_version)
        })


class LocalProjectManifest(namedtuple("LocalProjectManifest", ["env"])):
    @staticmethod
    def load(path):
        obj = read_yaml(path)
        env = obj["env"]
        return LocalProjectManifest(env=env)

    def save(self, path):
        write_yaml(path, {"env": self.env})
