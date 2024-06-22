from collections import deque
import sys
input = lambda: sys.stdin.readline().strip()

def main():
    ops = ('+', '-', '*', '/')
    p = list(map(int, input().split()))
    op_by_order = ['' for _ in range(4)]
    for i in range(4):
        op_by_order[p[i] - 1] = ops[i]
    s = input()
    parsed = []
    num_idx = 0
    for i in range(len(s)):
        if s[i] in ('+', '-', '*', '/'):
            parsed.append(int(s[num_idx:i]))
            parsed.append(s[i])
            num_idx = i + 1
    parsed.append(int(s[num_idx:]))

    parsed.reverse()

    for i in range(3, -1, -1):
        op = op_by_order[i]
        for j in range(len(parsed)):
            if parsed[j] == op:
                a = parsed[j - 1]
                b = parsed[j + 1]
                if op == '+':
                    res = a + b
                elif op == '-':
                    res = a - b
                elif op == '*':
                    res = a * b
                elif op == '/':
                    res = abs(a) // abs(b) * (1 if a * b >= 0 else -1)
                parsed[j - 1] = ''
                parsed[j] = ''
                parsed[j + 1] = res

        parsed_new = []
        for p in parsed:
            if p != '':
                parsed_new.append(p)
        parsed = parsed_new

    print(parsed[0])

if __name__ == "__main__":
    main()