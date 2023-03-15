from abc import ABC, abstractmethod
from collections import namedtuple
from isopy_lib.__fs_private__ import dir_path, file_path
from isopy_lib.features import GENERATE_CMD_WRAPPERS
from psutil import Process
import os
import platform
import shlex
import shutil


PYTHON_PROGRAMS = [
    "python3",
    "python",
    "pip3",
    "pip"
]


POWERSHELL_PATH = file_path(
    os.getenv("WINDIR"),
    "System32",
    "WindowsPowerShell",
    "v1.0",
    "powershell.exe")


CMD_PATH = os.getenv("ComSpec")


class Platform(ABC, namedtuple("Platform", ["name", "home_dir_meta", "home_dir", "python_executable_name", "python_bin_dirs", "asset_os", "asset_flavour"])):
    def __str__(self):
        return self.name

    @abstractmethod
    def path_env(self, env_config, export=True): raise NotImplementedError()

    @abstractmethod
    def exec(self, command=None, path_dirs=[], extra_env={}, prune_paths=False):
        raise NotImplementedError()


class UnixPlatform(Platform):
    def path_env(self, env_config, export=True):
        bin_dirs = [
            dir_path(env_config.path, "..", env_config.python_dir, x)
            for x in self.python_bin_dirs
        ]
        prefix = "export " if export else ""
        return \
            f"{prefix}PATH=" + \
            "".join([f"{d}{os.pathsep}" for d in bin_dirs]) + \
            "$PATH"

    def exec(self, command=None, path_dirs=[], extra_env={}, prune_paths=False):
        e = dict(os.environ)
        e["PATH"] = make_paths_str(
            e["PATH"], path_dirs, prune_paths=prune_paths)
        e.update(extra_env)

        if command is None:
            prog = os.getenv("SHELL")
            args = [prog]
        else:
            prog = command[0]
            args = command

        os.execlpe(prog, *args, e)


class WindowsPlatform(Platform):
    def path_env(self, env_config, export=True):
        bin_dirs = [
            dir_path(env_config.path, "..", env_config.python_dir, x)
            for x in self.python_bin_dirs
        ]
        if GENERATE_CMD_WRAPPERS:
            return \
                f"set PATH=" + \
                "".join([f"{d}{os.pathsep}" for d in bin_dirs]) + \
                "%PATH%"
        else:
            return \
                "$env:Path = " + \
                "".join([f"'{d}' + '{os.pathsep}' + " for d in bin_dirs]) + \
                "$env:Path"

    def exec(self, command=None, path_dirs=[], extra_env={}, prune_paths=False):
        def get_shell():
            parent_process = Process(os.getppid())
            c = parent_process.cmdline()
            if len(c) == 1:
                p = c[0].lower()
                if p == POWERSHELL_PATH.lower() or p == CMD_PATH.lower():
                    return c[0]
            raise NotImplementedError(f"Unsupported shell {c[0]}")

        shell = get_shell()
        os.environ["PATH"] = make_paths_str(
            os.getenv("PATH"),
            path_dirs,
            prune_paths=prune_paths)
        os.environ.update(extra_env)

        if command is None:
            os.system(f"\"{shell}\" -NoExit -NoProfile")
        else:
            os.system(f"\"{shell}\" -NoProfile -Command {shlex.join(command)}")


# Only works on Unix-like file systems
def prune_search_paths(paths):
    if PLATFORM == WINDOWS:
        raise NotImplementedError(f"Unsupported platform {PLATFORM}")

    cleaned_paths = []
    for p in paths:
        python_programs = [
            x
            for x in
            [shutil.which(x, path=p) for x in PYTHON_PROGRAMS]
            if x is not None and not x.startswith("/usr/")
        ]
        if len(python_programs) == 0:
            cleaned_paths.append(p)
    return cleaned_paths


def make_paths_str(paths_str, dirs, prune_paths=False):
    paths = [] if paths_str is None else paths_str.split(os.pathsep)

    cleaned_paths = prune_search_paths(paths) if prune_paths else paths

    for d in dirs:
        if d in cleaned_paths:
            cleaned_paths.remove(d)

    return os.pathsep.join(dirs + cleaned_paths)


LINUX = UnixPlatform(
    name="Linux",
    home_dir_meta="$HOME",
    home_dir=os.path.expanduser("~"),
    python_executable_name="python3",
    python_bin_dirs=["bin"],
    asset_os="linux",
    asset_flavour="gnu")
MACOS = UnixPlatform(
    name="macOS",
    home_dir_meta="$HOME",
    home_dir=os.path.expanduser("~"),
    python_executable_name="python3",
    python_bin_dirs=["bin"],
    asset_os="darwin",
    asset_flavour=None)
WINDOWS = WindowsPlatform(
    name="Windows",
    home_dir_meta="%USERPROFILE%",
    home_dir=os.path.expanduser("~"),
    python_executable_name="python",
    python_bin_dirs=[".", "Scripts"],
    asset_os="windows",
    asset_flavour="msvc")


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
