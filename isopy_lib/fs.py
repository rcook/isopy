from contextlib import contextmanager
from isopy_lib.errors import ReportableError
from tempfile import NamedTemporaryFile
import os
import shutil


def split_at_ext(s, exts):
    for ext in exts:
        if s.endswith(ext):
            s_len = len(s)
            ext_len = len(ext)
            temp = s_len - ext_len
            return s[0:temp], s[temp:]
    raise ValueError(f"Name {s} has unknown extension")


def copy_file(source, target, overwrite=False):
    # Obviously something could happen between the check and the
    # copy, but I'm not going to be clever...
    if not overwrite and os.path.exists(target):
        raise ReportableError(f"Target file {target} already exists")

    target_dir = os.path.dirname(target)
    os.makedirs(target_dir, exist_ok=True)
    shutil.copyfile(source, target)


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
