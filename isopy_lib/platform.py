from enum import Enum, auto, unique
import platform


@unique
class Platform(Enum):
    LINUX = auto()
    MACOS = auto()
    WINDOWS = auto()

    @classmethod
    def current(cls):
        os = platform.system().lower()
        if os == "linux":
            return cls.LINUX
        elif os == "darwin":
            return cls.MACOS
        elif os == "windows":
            return cls.WINDOWS
        else:
            raise NotImplementedError(f"Unsupported OS \"{os}\"")
