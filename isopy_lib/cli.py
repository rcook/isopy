from isopy_lib.version import Version


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
