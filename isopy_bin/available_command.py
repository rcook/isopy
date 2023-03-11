from isopy_lib.asset import get_assets
from isopy_lib.xprint import xprint
import colorama


def do_available(ctx, asset_filter):
    assets = get_assets(ctx=ctx, asset_filter=asset_filter)

    attrs = ["os", "arch", "tag_name", "python_version"]

    max_lens = [0] * len(attrs)
    for asset in assets:
        for i in range(0, len(attrs)):
            temp = len(str(getattr(asset, attrs[i])))
            if temp > max_lens[i]:
                max_lens[i] = temp

    for asset in assets:
        xprint(colorama.Fore.YELLOW, end="")
        for i in range(0, len(attrs)):
            if i > 0:
                print("  ", end="")
            print(str(getattr(asset, attrs[i])).ljust(max_lens[i]), end="")
        xprint()
