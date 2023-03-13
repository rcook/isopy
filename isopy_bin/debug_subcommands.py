from isopy_bin.commands.debug import do_debug
from isopy_lib.cli import add_subcommand, auto_description


def add_debug_subcommands(helper, subparsers):
    p = add_subcommand(
        subparsers,
        "debug",
        **auto_description("show debugging information"),
        func=lambda ctx, args: do_debug(ctx=ctx))
    helper.add_common_args(parser=p)
