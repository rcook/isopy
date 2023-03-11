from isopy_lib.fs import file_path
import hashlib
import os


def make_checksum_file_path(tag_name):
    return file_path(
        __file__,
        "..",
        "..",
        "sha256sums",
        f"{tag_name}.sha256sums")


def verify_checksum(file_path, checksum_file_path, file_name_key):
    with open(file_path, "rb") as f:
        file_bytes = f.read()

    with open(checksum_file_path, "rt") as f:
        lines = f.readlines()

    parts = [line.strip().split(" ") for line in lines]
    hash_map = {k: v for v, _, k in parts}
    expected_digest = hash_map[file_name_key]
    digest = hashlib.sha256(file_bytes).hexdigest()
    return digest == expected_digest
