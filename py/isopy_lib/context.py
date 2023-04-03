from collections import namedtuple


Context = namedtuple("Context", [
    "cwd",
    "logger",
    "cache_dir"
])
