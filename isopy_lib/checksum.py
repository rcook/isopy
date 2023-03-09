import hashlib
import os


def verify_checksum(file_path, checksum_file_path):
    with open(checksum_file_path, "rt") as f:
        lines = f.readlines()

    with open(file_path, "rb") as f:
        file_bytes = f.read()

    parts = [line.strip().split(" ") for line in lines]
    hash_map = {k: v for v, _, k in parts}

    file_name = os.path.basename(file_path)
    expected_digest = hash_map[file_name]
    digest = hashlib.sha256(file_bytes).hexdigest()
    return digest == expected_digest
