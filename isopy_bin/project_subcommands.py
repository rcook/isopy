from isopy_bin.commands.new import do_new
from isopy_lib.asset import AssetFilter
from isopy_lib.cli import \
    add_python_version_positional_arg, \
    add_subcommand, \
    add_tag_arg, \
    auto_description


def add_project_subcommands(helper, subparsers):
    p = add_subcommand(
        subparsers,
        "new",
        **auto_description("create project configuration"),
        func=lambda ctx, args: do_new(
            ctx=ctx,
            asset_filter=AssetFilter.default(
                tag=args.tag,
                python_version=args.python_version)))
    helper.add_common_args(parser=p)
    add_python_version_positional_arg(parser=p)
    add_tag_arg(parser=p)
