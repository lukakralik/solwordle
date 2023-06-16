from sys import stdin, argv
import re

exclude = ['%', '#', '*', '(', ')', ',', '!', '.', '$', '-', '_', '/', '\'']+list("&")

def check(s: str) -> bool:
    if bool(re.search(r'\d', s)):
        return False
    for char in exclude:
        if char in s:
            return False
    return True

def main():
    """
    USAGE:

    python filter.py $length$ < $dictionary location$ > $output location$

    dictionaries are located in /words under in format <language>(len).txt
    """
    req_len = int(argv[1]) + 1
    for line in stdin:
        if check(str(line)) and len(line) == req_len:
            print(line
                  .strip()
                  .upper()
                  )

if __name__ == "__main__":
    main()
