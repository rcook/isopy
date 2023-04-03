from isopy_lib.asset import AssetFilter
from isopy_lib.cli import \
    add_detailed_arg, \
    add_env_arg, \
    add_env_positional_arg, \
    add_force_arg, \
    add_python_version_positional_arg, \
    add_subcommand, \
    add_tag_arg, \
    auto_description
from isopy_bin.commands.create import do_create
from isopy_bin.commands.info import do_info
from isopy_bin.commands.init import do_init
from isopy_bin.commands.list import do_list
from isopy_bin.commands.use import do_use


def add_env_subcommands(helper, subparsers):
    p = add_subcommand(
        subparsers,
        "list",
        **auto_description("show environments"),
        func=lambda ctx, args: do_list(ctx=ctx, detailed=args.detailed))
    helper.add_common_args(parser=p)
    add_detailed_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "init",
        **auto_description("initialize environment for current project"),
        func=lambda ctx, args: do_init(ctx=ctx))
    helper.add_common_args(parser=p)

    p = add_subcommand(
        subparsers,
        "use",
        **auto_description("specify environment to use for current directory"),
        func=lambda ctx, args: do_use(ctx=ctx, env=args.env, force=args.force))
    helper.add_common_args(parser=p)
    add_env_positional_arg(parser=p)
    add_force_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "create",
        **auto_description("create named environment"),
        func=lambda ctx, args: do_create(
            ctx=ctx,
            env=args.env,
            asset_filter=AssetFilter.default(
                tag=args.tag,
                python_version=args.python_version)))
    helper.add_common_args(parser=p)
    add_env_positional_arg(parser=p)
    add_python_version_positional_arg(parser=p)
    add_tag_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "info",
        **auto_description("information about current environment"),
        func=lambda ctx, args: do_info(ctx=ctx, env=args.env))
    helper.add_common_args(parser=p)
    add_env_arg(parser=p)
