from contextlib import contextmanager
from tempfile import NamedTemporaryFile
import os


def dir_path(*args):
    return os.path.abspath(os.path.join(*args))


def file_path(*args):
    return os.path.abspath(os.path.join(*args))


def split_at_ext(s, exts):
    for ext in exts:
        if s.endswith(ext):
            s_len = len(s)
            ext_len = len(ext)
            temp = s_len - ext_len
            return s[0:temp], s[temp:]
    raise ValueError(f"Name {s} has unknown extension")


def move_file(source, target):
    target_dir = os.path.dirname(target)
    os.makedirs(target_dir, exist_ok=True)
    os.rename(source, target)


@contextmanager
def named_temporary_file(*args, **kwargs):
    t = None
    try:
        t = NamedTemporaryFile(*args, **kwargs)
        yield t
    finally:
        if t is not None:
            try:
                t.close()
            except FileNotFoundError:
                pass
