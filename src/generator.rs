use rand::Rng;
use std::fs;
use std::io::{BufWriter, Write};
use std::time::Instant;

pub fn generate(size: usize, min_value: usize, max_value: usize, solvable: Option<bool>) {
    let start = Instant::now();
    let mut rng = rand::thread_rng();

    let solvable = solvable.unwrap_or_else(|| rng.gen_bool(0.5));

    let gen_start = Instant::now();
    let values: Vec<usize> = (0..size)
        .map(|_| rng.gen_range(min_value..=max_value))
        .collect();
    println!("Values generated in {:.3?}", gen_start.elapsed());

    let target: usize = if solvable {
        let k = rng.gen_range(1..=size);
        let indices = rand::seq::index::sample(&mut rng, size, k);
        indices.iter().map(|i| values[i]).sum()
    } else {
        rng.gen_range(1..=(size * max_value))
    };

    let label = if solvable { "solvable" } else { "unknown" };
    let filename = format!("test/input-{}-{}.txt", size, label);

    fs::create_dir_all("test").expect("Failed to create test directory");
    let file = fs::File::create(&filename).expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", size).unwrap();
    writeln!(writer, "{}", target).unwrap();

    for (i, v) in values.iter().enumerate() {
        if i > 0 {
            write!(writer, " ").unwrap();
        }
        write!(writer, "{}", v).unwrap();
    }
    writeln!(writer).unwrap();

    let elapsed = start.elapsed();
    println!(
        "Generated {}: size={}, target={} ({:.3?})",
        filename, size, target, elapsed
    );
}
