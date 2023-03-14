from contextlib import contextmanager
from isopy_lib.__fs_private__ import dir_path, file_path
from isopy_lib.platform import LINUX, MACOS, PLATFORM, WINDOWS
from tempfile import NamedTemporaryFile
import os


def split_at_ext(s, exts):
    for ext in exts:
        if s.endswith(ext):
            s_len = len(s)
            ext_len = len(ext)
            temp = s_len - ext_len
            return s[0:temp], s[temp:]
    raise ValueError(f"Name {s} has unknown extension")


def move_file(source, target, overwrite=False):
    target_dir = os.path.dirname(target)
    os.makedirs(target_dir, exist_ok=True)

    if PLATFORM == LINUX or PLATFORM == MACOS:
        if overwrite:
            os.rename(source, target)
        else:
            os.link(source, target)
            os.unlink(source)
    elif PLATFORM == WINDOWS:
        if overwrite:
            os.replace(source, target)
        else:
            os.rename(source, target)
    else:
        raise NotImplementedError(f"Unsupported platform {PLATFORM}")


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
