import os
import requests


def download_file(url, local_path):
    local_dir = os.path.dirname(local_path)
    os.makedirs(local_dir, exist_ok=True)
    response = requests.get(url, allow_redirects=True)
    with open(local_path, "wb") as f:
        f.write(response.content)
