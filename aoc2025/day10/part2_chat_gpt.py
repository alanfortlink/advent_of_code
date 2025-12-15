import re
import sys
from pathlib import Path

try:
    import pulp
except ImportError:
    print("Install pulp: pip install pulp", file=sys.stderr)
    raise

LINE_RE = re.compile(r"\{([^}]*)\}")

def parse_line(line: str):
    m = LINE_RE.search(line)
    if not m:
        return None
    targets = [int(x.strip()) for x in m.group(1).split(",") if x.strip()]

    buttons = []
    for part in re.findall(r"\(([^)]*)\)", line):
        idxs = [int(x.strip()) for x in part.split(",") if x.strip()]
        buttons.append(idxs)

    return targets, buttons

def solve_machine(targets, buttons):
    m = len(targets)
    n = len(buttons)

    A = [[0] * n for _ in range(m)]
    for j, idxs in enumerate(buttons):
        for i in idxs:
            if 0 <= i < m:
                A[i][j] = 1

    prob = pulp.LpProblem("factory_part2", pulp.LpMinimize)
    x = [pulp.LpVariable(f"x_{j}", lowBound=0, cat="Integer") for j in range(n)]

    prob += pulp.lpSum(x)

    for i in range(m):
        prob += (pulp.lpSum(A[i][j] * x[j] for j in range(n)) == targets[i]), f"c_{i}"

    status = prob.solve(pulp.PULP_CBC_CMD(msg=False))
    if status != pulp.LpStatusOptimal:
        return None

    return int(pulp.value(prob.objective))

def main(path: str):
    total = 0
    per_machine = []

    text = Path(path).read_text(encoding="utf-8").strip().splitlines()
    for line in text:
        parsed = parse_line(line)
        if parsed is None:
            continue
        targets, buttons = parsed
        best = solve_machine(targets, buttons)
        if best is None:
            raise RuntimeError("NO SOLUTION")
        per_machine.append(best)
        total += best

    print("Total:", total)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python part2.py <input.txt>")
        raise SystemExit(1)
    main(sys.argv[1])
