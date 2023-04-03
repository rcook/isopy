from isopy_bin.commands.debug import do_debug
from isopy_lib.cli import add_detailed_arg, add_subcommand, auto_description


def add_debug_subcommands(helper, subparsers):
    p = add_subcommand(
        subparsers,
        "debug",
        **auto_description("show debugging information"),
        func=lambda ctx, args: do_debug(ctx=ctx, detailed=args.detailed))
    helper.add_common_args(parser=p)
    add_detailed_arg(parser=p, default=False)
