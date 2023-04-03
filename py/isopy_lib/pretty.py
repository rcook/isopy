from isopy_lib.xprint import xprint
import colorama


def get_value(obj, key):
    temp = getattr(obj, key, None)
    if temp is not None:
        return temp

    return obj[key]


def get_fields(obj):
    fields = getattr(obj, "_fields", None)
    if fields is not None:
        return fields

    keys_func = getattr(obj, "keys", None)
    if keys_func is not None:
        return list(keys_func())

    raise NotImplementedError(f"Cannot get fields for type {type(obj)}")


def get_widths(fields, items):
    widths = [len(x) for x in fields]
    for item in items:
        for i in range(0, len(fields)):
            temp = len(str(get_value(item, fields[i])))
            if temp > widths[i]:
                widths[i] = temp
    return widths


def show_table(items, fields=None):
    item_count = len(items)
    if item_count == 0:
        return

    if fields is None:
        fields = get_fields(items[0])

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
            print(str(get_value(item, fields[i])).ljust(widths[i]), end="")
        xprint()
