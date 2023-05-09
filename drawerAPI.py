import os
import asciidrawer as ass


def _character_checker(s: str) -> str:
    """Checks if given characters are ascii and destroys the duplicates

    Parameters:
    s (str): String of characters to pass further

    Returns:
    chars str: string of given symbols without duplicates

   """
    if type(s) != str:
        raise TypeError("given argument should be \'str\' type")
    if not s.isascii():
        raise ValueError("given string \"{}\" contains non-ASCII characters".format(s))
    chars = []
    for char in s:
        if char not in chars:
            chars.append(char)
    ret = ""
    for i in chars:
        ret += str(i)
    return ret


def _check_cfg_values(path_from: str, grid_x: int, grid_y: int):
    """Throws exceptions

        Parameters:
        path_from (str): path to the file from which to create ASCII art
        grid_x (int): x length of the output ASCII art
        grid_y (int): y length of the output ASCII art
        use_y (bool): use grid_y to determine size of the output

    """
    if not os.path.isfile(path_from):
        raise OSError("file \"{}\" does not exist".format(path_from))
    if grid_x <= 0 or grid_x >= 200:
        raise ValueError("x size of the grid should be between 1 and 200")
    if grid_y <= 0 or grid_y >= 200:
        raise ValueError("y size of the grid should be between 1 and 200")


def draw_ascii(path_from, symbols, grid_x, grid_y, normalize=True):
    """Draws image

        Parameters:
        path_from (str): path to the file from which to create ASCII art
        grid_x (int): x length of the output ASCII art
        grid_y (int): y length of the output ASCII art
        normalize (bool): normalize image to improve contrast

    """
    symbols_filtered = _character_checker(symbols)
    _check_cfg_values(path_from, grid_x, grid_y)
    return ass.draw(path_from, symbols_filtered, grid_x, grid_y, normalize)
