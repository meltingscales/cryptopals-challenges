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


def int2binstr(n: int, minLength=8) -> str:

    res = ''
    while n > 0:
        res = str(n & 1) + res  # prepend odd bit to avoid having to reverse it
        n = n >> 1  # divide by 2

    while len(res) < minLength:
        res = '0' + res

    return res


# for number in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]:
#     print(f"{number} = {int2binstr(number)}")

def base64Encode(data: List[int]) -> List[int]:
    padCount = len(data) % 3

    print(f'we need to pad input data {padCount} times')

    # for i in range(0, len(data), 1):
    for i in range(0, len(data), 3):

        

        threeOctets = data[i+2] + (data[i+1] << 8) + (data[i] << 16)
        print(f"3 octets: {int2binstr(threeOctets, minLength=24)}")

        byte = data[i]

        print(f'"{byte:3d}" = "{chr(byte):1s}" = {int2binstr(byte):8s}')

    return


if __name__ == '__main__':

    with open('input', 'rb') as f:
        data: List[int] = []

        for byte in f.read():
            data.append(byte)

    encodedData = base64Encode(data)

    # print(encodedData)
