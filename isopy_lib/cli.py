from isopy_lib.version import Version
from operator import itemgetter
import logging


def auto_description(help):
    if len(help) > 0:
        return {
            "help": help,
            "description": help[0].upper() + help[1:]
        }
    else:
        return {}


def add_env_positional_arg(parser):
    parser.add_argument(
        "env",
        metavar="ENV",
        help="environment")


def add_env_arg(parser):
    parser.add_argument(
        "--env",
        "-e",
        metavar="ENV",
        help="environment")


def add_python_version_positional_arg(parser):
    parser.add_argument(
        "python_version",
        metavar="PYTHON_VERSION",
        type=Version.parse,
        help="Python version")


def add_log_level_arg(parser):
    m = logging.getLevelNamesMapping()
    m.pop("NOTSET")
    temp0 = list(sorted(m.items(), key=itemgetter(0)))
    temp1 = list(sorted(temp0, key=itemgetter(1), reverse=True))
    choices = list(map(lambda x: x[0].lower(), temp1))
    parser.add_argument(
        "--log-level",
        "-l",
        metavar="LOG_LEVEL",
        choices=choices,
        type=str,
        help=f"logging level (one of {', '.join(choices)})")
