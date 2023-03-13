from enum import Enum, unique
from psutil import Process
import os
import platform


@unique
class Platform(Enum):
    LINUX = "$HOME", os.path.expanduser("~"), "python3", ["bin"]
    MACOS = "$HOME", os.path.expanduser("~"), "python3", ["bin"]
    WINDOWS = "%USERPROFILE%", os.path.expanduser("~"), "python", [
        ".", "Scripts"]

    def __new__(cls, *args, **kwargs):
        value = len(cls.__members__) + 1
        obj = object.__new__(cls)
        obj._value_ = value
        return obj

    def __init__(self, home_dir_meta, home_dir, python_executable_name, bin_dirs):
        self.__home_dir_meta = home_dir_meta
        self.__home_dir = home_dir
        self.__python_executable_name = python_executable_name
        self.__bin_dirs = bin_dirs

    @property
    def home_dir_meta(self): return self.__home_dir_meta

    @property
    def home_dir(self): return self.__home_dir

    @property
    def python_executable_name(self): return self.__python_executable_name

    @property
    def bin_dirs(self): return self.__bin_dirs

    @classmethod
    def current(cls):
        p = platform.system().lower()
        if p == "linux":
            return cls.LINUX
        elif p == "darwin":
            return cls.MACOS
        elif p == "windows":
            return cls.WINDOWS
        else:
            raise NotImplementedError(f"Unsupported platform {p}")


def make_paths_str(paths_str, dirs):
    paths = [] if paths_str is None else paths_str.split(os.pathsep)

    for d in dirs:
        if d in paths:
            paths.remove(d)

    return os.pathsep.join(dirs + paths)


def get_windows_shell():
    parent_process = Process(os.getppid())
    c = parent_process.cmdline()
    if len(c) == 1 and c[0].endswith("powershell.exe"):
        return c[0]
    else:
        raise NotImplementedError(f"Unsupported shell {c[0]}")


def exec(command=None, path_dirs=[], extra_env={}):
    c = Platform.current()
    if c in [Platform.LINUX, Platform.MACOS]:
        e = dict(os.environ)
        e["PATH"] = make_paths_str(e["PATH"], path_dirs)
        e.update(extra_env)

        if command is None:
            prog = os.getenv("SHELL")
            args = [prog]
        else:
            prog = command[0]
            args = command

        os.execlpe(prog, *args, e)
    elif c == Platform.WINDOWS:
        shell = get_windows_shell()
        os.environ["PATH"] = make_paths_str(os.getenv("PATH"), path_dirs)
        os.environ.update(extra_env)
        os.system(f"\"{shell}\" -NoExit -NoProfile")
    else:
        raise NotImplementedError(f"Unsupported platform {c}")
