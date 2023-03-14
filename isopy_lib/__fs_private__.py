import os


def dir_path(*args):
    return os.path.abspath(os.path.join(*args))


def file_path(*args):
    return os.path.abspath(os.path.join(*args))
