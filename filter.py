from sys import stdin
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
    for line in stdin:
        if check(str(line)) and len(line) == 6:
            print(line
                  .strip()
                  .upper()
                  )

if __name__ == "__main__":
    main()
