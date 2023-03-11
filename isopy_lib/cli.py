from isopy_lib.version import Version
from operator import itemgetter
import argparse
import logging
import re


ENV_RE = re.compile("^([A-Za-z0-9-_]+)$")


def env_type(s):
    m = ENV_RE.match(s)
    if m is None:
        raise argparse.ArgumentTypeError(f"invalid environment name {s}")

    return s


def add_subcommand(subparsers, *args, func, **kwargs):
    parser = subparsers.add_parser(*args, **kwargs)
    parser.set_defaults(func=func)
    return parser


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
        type=env_type,
        help="environment")


def add_env_arg(parser):
    parser.add_argument(
        "--env",
        "-e",
        metavar="ENV",
        type=env_type,
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
        default="info",
        help=f"logging level (one of {', '.join(choices)})")


def add_cache_dir_arg(parser, default):
    parser.add_argument(
        "--cache-dir",
        "-c",
        metavar="CACHE_DIR",
        default=default,
        help=f"cache directory (default: {default})")


def add_tag_name_arg(parser):
    parser.add_argument(
        "--tag-name",
        "-t",
        metavar="TAG_NAME",
        type=str,
        required=False,
        help="tag name")


def add_python_version_arg(parser):
    parser.add_argument(
        "--python-version",
        "-v",
        metavar="PYTHON_VERSION",
        type=Version.parse,
        required=False,
        help="Python version")


def add_force_arg(parser):
    parser.add_argument(
        "--force",
        "-f",
        metavar="FORCE",
        action=argparse.BooleanOptionalAction,
        help="force overwrite of output files")
