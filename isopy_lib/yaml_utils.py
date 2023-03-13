import os
import yaml


def nullable_str(obj):
    return None if obj is None else str(obj)


def read_yaml(path):
    with open(path, "rt") as f:
        return yaml.load(f, Loader=yaml.SafeLoader)


def write_yaml(path, obj):
    dir = os.path.dirname(path)
    os.makedirs(dir, exist_ok=True)
    with open(path, "xt") as f:
        yaml.dump(obj, f)
