from isopy_lib.fs import dir_path
from isopy_lib.platform import Platform
from psutil import Process
import os


ISOPY_ENV_VAR_NAME = "ISOPY_ENV"


def get_python_executable_name():
    c = Platform.current()
    if c in [Platform.LINUX, Platform.MACOS]:
        return "python3"
    elif c == Platform.WINDOWS:
        return "python"
    else:
        raise NotImplementedError(f"Unsupported platform {c}")


def get_windows_shell():
    parent_process = Process(os.getppid())
    c = parent_process.cmdline()
    if len(c) == 1 and c[0].endswith("powershell.exe"):
        return c[0]
    else:
        raise NotImplementedError(f"Unsupported shell {c[0]}")


def make_paths_str(paths_str, dirs):
    paths = [] if paths_str is None else paths_str.split(os.pathsep)

    for d in dirs:
        if d in paths:
            paths.remove(d)

    return os.pathsep.join(dirs + paths)


def in_isopy_shell(ctx):
    return os.getenv(ISOPY_ENV_VAR_NAME) is not None


def start_isopy_shell(ctx, env_config):
    label = env_config.name or env_config.dir_config_path
    python_dir = env_config.make_python_dir(ctx=ctx)

    c = Platform.current()
    if c in [Platform.LINUX, Platform.MACOS]:
        bin_dir = dir_path(python_dir, "bin")
        shell = os.getenv("SHELL")

        e = dict(os.environ)
        e["PATH"] = make_paths_str(e["PATH"], [bin_dir])
        e[ISOPY_ENV_VAR_NAME] = label

        os.execlpe(shell, shell, e)
    elif c == Platform.WINDOWS:
        scripts_dir = dir_path(python_dir, "Scripts")
        shell = get_windows_shell()

        os.environ["PATH"] = make_paths_str(
            os.getenv("PATH"),
            [python_dir, scripts_dir])
        os.environ[ISOPY_ENV_VAR_NAME] = label

        os.system(f"\"{shell}\" -NoExit -NoProfile")
    else:
        raise NotImplementedError(f"Unsupported platform {c}")
