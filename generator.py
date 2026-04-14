import random


def generate(SIZE, MAX_VALUE=1000):

    values = [random.randint(1, MAX_VALUE) for _ in range(SIZE)]
    target = sum(random.sample(values, k=random.randint(1, SIZE)))

    filename = f"test/input-s{SIZE}.txt"

    with open(filename, "w") as f:
        f.write(f"{SIZE}\n")
        f.write(f"{target}\n")
        f.write(" ".join(str(v) for v in values) + "\n")

    print(f"Generated {filename}: size={SIZE}, target={target}")


if __name__ == "__main__":
    generate(2000)
