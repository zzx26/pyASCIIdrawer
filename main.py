import drawerAPI


def main():
    path_cat = r"resources/DB.png"
    out = drawerAPI.draw_ascii(path_cat, " .,qwe123pQWERTYOP", 60, 110, normalize=False)
    print(out)


if __name__ == '__main__':
    main()
