from sys import argv
from os import getcwd
from requests import get

get_url = lambda day: f"https://adventofcode.com/2021/day/{day}/input"
destination_dir = ".\\data\\"

if __name__ == "__main__":
    if not len(argv) == 2:
        print("invalid number of arguments")
        exit()

    try:
        day = int(argv[1])
    except:
        print("day need be a number")
    
    if not (0 < day < 26):
        print("invalid day")

    url = get_url(day)
    with open(".session", "r") as f:
        session = f.readline()
    cookies = {"session": session}
    ret = get(url, cookies=cookies, allow_redirects=True)
    with open(f"{destination_dir}day{day}.txt", "w") as f:
        f.write(ret.text.strip())

    print(f"{url} -> {destination_dir}day{day}.txt")
    for i, l in enumerate(ret.text.splitlines()):
        if i > 10: break
        else: print(l)
    print("...")
