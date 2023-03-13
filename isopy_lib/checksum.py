from isopy_lib.fs import dir_path, file_path
import hashlib


CHECKSUM_DIR = dir_path(
    __file__,
    "..",
    "..",
    "sha256sums")


def make_checksum_file_path(tag):
    return file_path(CHECKSUM_DIR, f"{tag}.sha256sums")


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
