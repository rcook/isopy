from isopy_lib.doc import DOC_BASE_URL, make_doc_url
from isopy_lib.features import OPEN_HELP_IN_BROWSER
from isopy_lib.platform import PLATFORM
from isopy_lib.version import Version
from operator import itemgetter
import argparse
import logging
import re
import sys


ENV_RE = re.compile("^([A-Za-z0-9-_]+)$")


class CapitalizedUsageHelpFormatter(argparse.HelpFormatter):
    def add_usage(self, usage, actions, groups, prefix=None):
        if prefix is None:
            prefix = "Usage: "
        return super().add_usage(usage, actions, groups, prefix)


class BrowserLaunchingArgumentParser(argparse.ArgumentParser):
    def print_help(self, file=None):
        if file is None:
            file = sys.stdout
        h = self.format_help()
        self._print_message(h, file)
        if OPEN_HELP_IN_BROWSER:
            for url in re.findall("(?:^|\s)(https?:\/\/[^\s]+)(?:$|\s)", h):
                if url != DOC_BASE_URL:
                    PLATFORM.shell_open(url)


def env_type(s):
    m = ENV_RE.match(s)
    if m is None:
        raise argparse.ArgumentTypeError(f"invalid environment name {s}")

    return s


def add_subcommand(subparsers, *args, func, **kwargs):
    name = args[0]
    doc_url = make_doc_url(command=name)
    parser = subparsers.add_parser(
        *args,
        epilog=f"See documentation at {doc_url}",
        **kwargs)
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
    temp0 = list(sorted(
        sorted(m.items(), key=itemgetter(0)),
        key=itemgetter(1), reverse=True))
    temp1 = {v: k.lower() for k, v in temp0}

    choices = list(map(lambda x: x[0].lower(), temp0))
    default = temp1[logging.INFO]

    parser.add_argument(
        "--log-level",
        "-l",
        metavar="LOG_LEVEL",
        choices=choices,
        type=str,
        default=default,
        help=f"logging level (one of {', '.join(choices)}) (default: {default})")


def add_cache_dir_arg(parser, default):
    parser.add_argument(
        "--cache-dir",
        "-c",
        metavar="CACHE_DIR",
        default=default,
        help=f"cache directory (default: {default})")


def add_tag_arg(parser):
    parser.add_argument(
        "--tag",
        "-t",
        metavar="TAG",
        type=str,
        required=False,
        help="python-build-standalone build tag")


def add_python_version_arg(parser):
    parser.add_argument(
        "--python-version",
        "-v",
        metavar="PYTHON_VERSION",
        type=Version.parse,
        required=False,
        help="Python version")


def add_force_arg(parser):
    default = False

    parser.add_argument(
        "--force",
        "-f",
        metavar="FORCE",
        action=argparse.BooleanOptionalAction,
        default=default,
        help=f"force overwrite of output files (default: {default})")


def add_refresh_arg(parser):
    default = False

    parser.add_argument(
        "--refresh",
        metavar="REFRESH",
        action=argparse.BooleanOptionalAction,
        default=default,
        help=f"force download a new assets list (default: {default})")


def add_detailed_arg(parser, default=True):
    parser.add_argument(
        "--detail",
        dest="detailed",
        metavar="DETAILED",
        action=argparse.BooleanOptionalAction,
        default=default,
        help=f"show detailed output (default: {default})")


def add_prune_paths_arg(parser):
    default = False

    parser.add_argument(
        "--prune",
        dest="prune_paths",
        metavar="PRUNE_PATHS",
        action=argparse.BooleanOptionalAction,
        default=default,
        help=f"aggressively prune system PATH directories containing other Pythons (default: {default})")


def add_quiet_arg(parser):
    default = False

    parser.add_argument(
        "--quiet",
        "-q",
        dest="quiet",
        metavar="QUIET",
        action=argparse.BooleanOptionalAction,
        default=default,
        help=f"quiet output of command (default: {default})")