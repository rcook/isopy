from isopy_lib.asset import AssetFilter
from isopy_lib.cli import \
    add_detailed_arg, \
    add_python_version_arg, \
    add_python_version_positional_arg, \
    add_refresh_arg, \
    add_subcommand, \
    add_tag_arg, \
    auto_description
from isopy_bin.commands.available import do_available
from isopy_bin.commands.download import do_download
from isopy_bin.commands.downloaded import do_downloaded


def add_asset_subcommands(helper, subparsers):
    p = add_subcommand(
        subparsers,
        "available",
        **auto_description("show Python packages available for download"),
        func=lambda ctx, args: do_available(
            ctx=ctx,
            asset_filter=AssetFilter.default(
                tag=args.tag,
                python_version=args.python_version),
            refresh=args.refresh))
    helper.add_common_args(parser=p)
    add_tag_arg(parser=p)
    add_python_version_arg(parser=p)
    add_refresh_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "download",
        **auto_description("download Python package"),
        func=lambda ctx, args: do_download(
            ctx=ctx,
            asset_filter=AssetFilter.default(
                tag=args.tag,
                python_version=args.python_version)))
    helper.add_common_args(parser=p)
    add_python_version_positional_arg(parser=p)
    add_tag_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "downloaded",
        **auto_description("show downloaded Python packages"),
        func=lambda ctx, args: do_downloaded(ctx=ctx, detailed=args.detailed))
    helper.add_common_args(parser=p)
    add_detailed_arg(parser=p)
