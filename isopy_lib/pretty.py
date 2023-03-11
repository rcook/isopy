from isopy_lib.xprint import xprint
import colorama


def get_widths(attrs, items):
    widths = [0] * len(attrs)
    for item in items:
        for i in range(0, len(attrs)):
            temp = len(str(getattr(item, attrs[i])))
            if temp > widths[i]:
                widths[i] = temp
    return widths


def show_item_table(attrs, items):
    widths = get_widths(attrs=attrs, items=items)
    for item in items:
        xprint(colorama.Fore.YELLOW, end="")
        for i in range(0, len(attrs)):
            if i > 0:
                print("  ", end="")
            print(str(getattr(item, attrs[i])).ljust(widths[i]), end="")
        xprint()
