use std::fs;

pub fn parse_input(file: &str) -> (usize, usize, Vec<usize>) {
    let contents = fs::read_to_string(file).expect("Failed to read input file");
    let lines: Vec<&str> = contents.lines().collect();

    let size: usize = lines[0].trim().parse().expect("Failed to parse size");
    let target: usize = lines[1].trim().parse().expect("Failed to parse target");
    let values: Vec<usize> = lines[2]
        .trim()
        .split_whitespace()
        .map(|v| v.parse().expect("Failed to parse value"))
        .collect();

    assert_eq!(
        values.len(),
        size,
        "Number of values doesn't match declared size"
    );

    (size, target, values)
}
