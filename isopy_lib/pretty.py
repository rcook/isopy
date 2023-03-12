from isopy_lib.xprint import xprint
import colorama


def get_widths(fields, items):
    widths = [len(x) for x in fields]
    for item in items:
        for i in range(0, len(fields)):
            temp = len(str(getattr(item, fields[i])))
            if temp > widths[i]:
                widths[i] = temp
    return widths


def show_table(items, fields=None):
    item_count = len(items)
    if item_count == 0:
        return

    if fields is None:
        fields = items[0]._fields

    widths = get_widths(fields=fields, items=items)
    xprint(colorama.Fore.LIGHTWHITE_EX, end="")
    for i in range(0, len(fields)):
        if i > 0:
            print("  ", end="")
        print(str(fields[i].upper()).ljust(widths[i]), end="")
    xprint()

    for item in items:
        xprint(colorama.Fore.YELLOW, end="")
        for i in range(0, len(fields)):
            if i > 0:
                print("  ", end="")
            print(str(getattr(item, fields[i])).ljust(widths[i]), end="")
        xprint()
