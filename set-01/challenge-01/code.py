# see https://en.wikipedia.org/wiki/Base64


def print_bits(i:int)->None:
    pass

from typing import List

with open('input', 'rb') as f:

    data: List[int] = []

    for byte in f.read():
        data.append(byte)
    
    padCount = len(data)%3

    print(f'pad {padCount} times')

    for i in range(0, len(data), 3):
        print(i)