from contextlib import contextmanager
from isopy_lib.platform import Platform
from tempfile import NamedTemporaryFile
import os


def get_home_dir_meta():
    c = Platform.current()
    if c == Platform.LINUX:
        return "$HOME"
    elif c == Platform.WINDOWS:
        return "%USERPROFILE"
    else:
        raise NotImplementedError()


def get_home_dir():
    return os.path.expanduser("~")


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
def named_temporary_file():
    p = None
    try:
        with NamedTemporaryFile(delete=False) as t:
            p = t.name
            assert os.path.isfile(p)
            t.close()
            assert os.path.isfile(p)
            yield t
    finally:
        if p is not None:
            try:
                os.unlink(p)
            except FileNotFoundError:
                pass
