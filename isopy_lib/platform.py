from enum import Enum, unique
import os
import platform


@unique
class Platform(Enum):
    LINUX = "$HOME", os.path.expanduser("~"), "python3"
    MACOS = "$HOME", os.path.expanduser("~"), "python3"
    WINDOWS = "%USERPROFILE%", os.path.expanduser("~"), "python"

    def __new__(cls, *args, **kwargs):
        value = len(cls.__members__) + 1
        obj = object.__new__(cls)
        obj._value_ = value
        return obj

    def __init__(self, home_dir_meta, home_dir, python_executable_name):
        self.__home_dir_meta = home_dir_meta
        self.__home_dir = home_dir
        self.__python_executable_name = python_executable_name

    @property
    def home_dir_meta(self): return self.__home_dir_meta

    @property
    def home_dir(self): return self.__home_dir

    @property
    def python_executable_name(self): return self.__python_executable_name

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
