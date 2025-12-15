import sys

input_file = sys.argv[1]
ranges = [[int(x) for x in rng.split("-")] for rng in open(input_file, 'r').read().split(",")]

def is_invalid_1(v):
    sv = str(v)
    if len(sv) % 2 != 0:
        return False

    m = len(sv) // 2
    lhs, rhs = sv[:m], sv[m:]

    return lhs == rhs

def is_invalid_2(v):
    sv = str(v)
    for i in range(1, len(sv)):
        if len(sv) % i != 0:
            continue
        chunks = set()
        s = sv
        while s != "":
            chunks.add(s[:i])
            s = s[i:]
            if len(chunks) > 1:
                break

        if len(chunks) == 1:
            return True

    return False

def part1():
    res = 0
    for rng in ranges:
        s, e = rng
        for v in range(s, e+1):
            if is_invalid_1(v):
                res += v

    print(res)

def part2():
    res = 0
    for rng in ranges:
        s, e = rng
        for v in range(s, e+1):
            if is_invalid_2(v):
                res += v

    print(res)


part1()
part2()
