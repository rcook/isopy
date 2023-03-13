from isopy_lib.version import Version


def parse_python_version_and_tag(s):
    parts = s.split("+")
    if len(parts) != 2:
        raise ValueError(f"Invalid Python version and tag name {s}")

    python_version_str, tag = parts
    return Version.parse(python_version_str), tag
