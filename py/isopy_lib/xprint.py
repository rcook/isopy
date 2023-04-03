import colorama


def xprint(*args, end=colorama.Style.RESET_ALL + "\n", sep="", **kwargs):
    print(
        *args,
        end=end,
        sep=sep,
        **kwargs)
