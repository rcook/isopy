from isopy_bin.commands.exec import do_exec
from isopy_bin.commands.shell import do_shell
from isopy_bin.commands.wrap import do_wrap
from isopy_lib.cli import \
    add_env_arg, \
    add_prune_paths_arg, \
    add_subcommand, \
    auto_description
import argparse


def add_shell_subcommands(helper, subparsers):
    p = add_subcommand(
        subparsers,
        "shell",
        **auto_description("open shell in environment"),
        func=lambda ctx, args: do_shell(ctx=ctx, env=args.env, prune_paths=args.prune_paths))
    helper.add_common_args(parser=p)
    add_env_arg(parser=p)
    add_prune_paths_arg(parser=p)

    p = add_subcommand(
        subparsers,
        "exec",
        **auto_description("run command in shell in environment"),
        func=lambda ctx, args: do_exec(ctx=ctx, env=args.env, command=args.command, prune_paths=args.prune_paths))
    helper.add_common_args(parser=p)
    add_env_arg(parser=p)
    add_prune_paths_arg(parser=p)
    p.add_argument(
        "command",
        nargs=argparse.REMAINDER,
        metavar="COMMAND",
        help="command to run and its arguments")

    p = add_subcommand(
        subparsers,
        "wrap",
        **auto_description("generate shell wrapper for Python script"),
        func=lambda ctx, args: do_wrap(
            ctx=ctx,
            env=args.env,
            wrapper_path=args.wrapper_path,
            script_path=args.script_path,
            base_dir=args.base_dir))
    helper.add_common_args(parser=p)
    add_env_arg(parser=p)
    p.add_argument(
        "wrapper_path",
        metavar="WRAPPER_PATH",
        type=helper.file_path_type,
        help="path to output wrapper script")
    p.add_argument(
        "script_path",
        metavar="SCRIPT_PATH",
        type=helper.file_path_type,
        help="path to Python script")
    p.add_argument(
        "base_dir",
        metavar="BASE_DIR",
        type=helper.dir_path_type,
        help="path to base directory")
