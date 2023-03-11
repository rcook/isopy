from isopy_lib.version import Version
import logging


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
    """
    parser.add_argument(
        "--log-level",
        "-l",
        metavar="LOG_LEVEL",
        choices=[],
        help="logging level")
    """
    pass
