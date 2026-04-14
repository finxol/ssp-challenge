import random


def generate(size, max_value=10000):
    values = [random.randint(1, max_value) for _ in range(size)]

    solvable = random.choice([True, False])

    if solvable:
        target = sum(random.sample(values, k=random.randint(1, size)))
    else:
        target = random.randint(1, size * max_value)

    filename = f"test/input-s{size}-{'solvable' if solvable else 'unknown'}.txt"

    with open(filename, "w") as f:
        f.write(f"{size}\n")
        f.write(f"{target}\n")
        f.write(" ".join(str(v) for v in values) + "\n")

    print(f"Generated {filename}: size={size}, target={target}")


if __name__ == "__main__":
    COUNT = 10

    for i in range(COUNT):
        SIZE = random.randint(10, 500)
        generate(SIZE)
