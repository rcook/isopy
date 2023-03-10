import os


def make_dir_path(*args):
    return os.path.abspath(os.path.join(*args))


def make_file_path(*args):
    return os.path.abspath(os.path.join(*args))


def split_at_ext(s, exts):
    for ext in exts:
        if s.endswith(ext):
            s_len = len(s)
            ext_len = len(ext)
            temp = s_len - ext_len
            return s[0:temp], s[temp:]
    raise ValueError(f"Name {s} has unknown extension")
