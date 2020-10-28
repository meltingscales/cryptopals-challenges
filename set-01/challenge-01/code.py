# see https://en.wikipedia.org/wiki/Base64
from typing import List


def largest_exponent(i: int) -> int:
    largest_exponent = 0
    exp = 0
    while (largest_exponent <= i):
        largest_exponent = 2 ** exp
        exp += 1

    # print(f'largest power is 2^{exp} or {largest_exponent} for {i}')
    return exp


def int2binstr(n: int) -> str:
    if n == 0:
        return '0'

    res = ''
    while n > 0:
        res = str(n & 1) + res  # prepend odd bit to avoid having to reverse it
        n = n >> 1  # divide by 2

    return res


# for number in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]:
#     print(f"{number} = {int2binstr(number)}")

if __name__ == '__main__':

    with open('input', 'rb') as f:
        data: List[int] = []

        for byte in f.read():
            data.append(byte)

        padCount = len(data) % 3

        print(f'pad input data {padCount} times')

        for i in range(0, len(data), 1):
        # for i in range(0, len(data), 3):
            byte=data[i]

            print(f'{byte:3d} = {chr(byte):1s} = {int2binstr(byte):8s}')
