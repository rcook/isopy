from collections import namedtuple
from enum import Enum, unique
from psutil import Process
import os
import platform


Platform = namedtuple("Platform", [
    "home_dir_meta",
    "home_dir",
    "python_executable_name",
    "python_bin_dirs",
    "exec"
])


def make_paths_str(paths_str, dirs):
    paths = [] if paths_str is None else paths_str.split(os.pathsep)

    for d in dirs:
        if d in paths:
            paths.remove(d)

    return os.pathsep.join(dirs + paths)


def exec_unix(command=None, path_dirs=[], extra_env={}):
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


def exec_windows(command=None, path_dirs=[], extra_env={}):
    def get_shell():
        parent_process = Process(os.getppid())
        c = parent_process.cmdline()
        if len(c) == 1 and c[0].endswith("powershell.exe"):
            return c[0]
        else:
            raise NotImplementedError(f"Unsupported shell {c[0]}")

    if command is None:
        pass
    else:
        raise NotImplementedError()

    shell = get_shell()
    os.environ["PATH"] = make_paths_str(os.getenv("PATH"), path_dirs)
    os.environ.update(extra_env)
    os.system(f"\"{shell}\" -NoExit -NoProfile")


LINUX = Platform(
    home_dir_meta="$HOME",
    home_dir=os.path.expanduser("~"),
    python_executable_name="python3",
    python_bin_dirs=["bin"],
    exec=exec_unix)
MACOS = Platform(
    home_dir_meta="$HOME",
    home_dir=os.path.expanduser("~"),
    python_executable_name="python3",
    python_bin_dirs=["bin"],
    exec=exec_unix)
WINDOWS = Platform(
    home_dir_meta="%USERPROFILE%",
    home_dir=os.path.expanduser("~"),
    python_executable_name="python",
    python_bin_dirs=[".", "Scripts"],
    exec=exec_windows)


def get_current_platform():
    p = platform.system().lower()
    if p == "linux":
        return LINUX
    elif p == "darwin":
        return MACOS
    elif p == "windows":
        return WINDOWS
    else:
        raise NotImplementedError(f"Unsupported platform {p}")


PLATFORM = get_current_platform()
