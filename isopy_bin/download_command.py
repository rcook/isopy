from isopy_lib.asset import
from isopy_lib.fs import file_path


def do_download(logger, cache_dir, env, asset_filter):
    asset = get_asset(
        logger=logger,
        cache_dir=cache_dir,
        env=env,
        asset_filter=asset_filter)

    python_path = file_path(cache_dir, "assets", asset.name)
    if os.path.isfile(python_path):
        logger.info(f"Package {python_path} is already available locally")
    else:
        raise NotImplementedError()
