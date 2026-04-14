import random
import time


def generate(size, max_value=1_000_000, solvable=random.choice([True, False])):
    start = time.time()
    values = [random.randint(1, max_value) for _ in range(size)]

    if solvable:
        target = sum(random.sample(values, k=random.randint(1, size)))
    else:
        target = random.randint(1, size * max_value)

    filename = f"test/input-{size}-{'solvable' if solvable else 'unknown'}.txt"

    with open(filename, "w") as f:
        f.write(f"{size}\n")
        f.write(f"{target}\n")
        f.write(" ".join(str(v) for v in values) + "\n")

    elapsed = time.time() - start
    print(f"Generated {filename}: size={size}, target={target} ({elapsed:.3f}s)")


if __name__ == "__main__":
    COUNT = 1

    for i in range(COUNT):
        SIZE = random.randint(1_000_000_000, 5_000_000_000)
        generate(SIZE, max_value=100_000_000, solvable=False)
