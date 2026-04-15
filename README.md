# Subset Sum Problem challenge

**This is a possibly out-of-date mirror of [tangled.org/finxol.eu/ssp-challenge](https://tangled.org/finxol.eu/ssp-challenge)**

Given a set of integers, the goal is to find a solution that sums to the target number.

## Usage

```sh
# Solve an instance
cargo run --release -- solve <input_file> --show-solution

# Generate a test instance
cargo run --release -- generate --size 1000 --max-value 100000 --solvable
```

### Solve options

| Flag | Description | Default |
|------|-------------|---------|
| `--show-solution N` | Display the found solution | false |

### Generate options

| Flag | Description | Default |
|------|-------------|---------|
| `--size N` | Number of values | random 1B-5B |
| `--min-value N` | Minimum value | 1 |
| `--max-value N` | Maximum value | 1,000,000 |
| `--count N` | Instances to generate | 1 |
| `--solvable` | Force solvable | random |
| `--unsolvable` | Force unsolvable | random |

## Input format

3-line text file such as:
```md
<the size of the input set>
<the target sum>
<the actual values of the set, separated by spaces>
```
