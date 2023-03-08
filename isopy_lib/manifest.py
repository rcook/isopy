from collections import namedtuple
from isopy_lib.version import Version
import json


class EnvManifest(namedtuple("EnvManifest", ["tag_name", "python_version", "python_dir"])):
    @staticmethod
    def load(path):
        with open(path, "rt") as f:
            obj = json.load(f)

        tag_name = obj["tag_name"]
        python_version = Version.parse(obj["python_version"])
        python_dir = obj["python_dir"]
        return EnvManifest(
            tag_name=tag_name,
            python_version=python_version,
            python_dir=python_dir)

    def save(self, path):
        with open(path, "wt") as f:
            f.write(json.dumps({
                "tag_name": self.tag_name,
                "python_version": str(self.python_version),
                "python_dir": self.python_dir
            }, indent=2, sort_keys=True))
