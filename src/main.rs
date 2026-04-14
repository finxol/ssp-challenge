mod generator;
mod parser;
mod ssp;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "solve" => solve(&args),
        "generate" => generate(&args),
        _ => {
            print_usage(&args[0]);
            std::process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    eprintln!("Usage:");
    eprintln!("  {program} solve <input_file>");
    eprintln!(
        "  {program} generate [--count N] [--size N] [--max-value N] [--solvable|--unsolvable]"
    );
}

fn solve(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: {} solve <input_file>", args[0]);
        std::process::exit(1);
    }

    let (size, target, values) = parser::parse_input(&args[2]);
    println!("Size: {}, Target: {}", size, target);

    let start = std::time::Instant::now();
    let result = ssp::ssp(size, target, values);
    let elapsed = start.elapsed();

    println!();

    if result.is_empty() {
        println!("No solution found");
    } else {
        println!("Found solution!");
    }

    println!();
    println!("Time: {:.3?}", elapsed);
}

fn generate(args: &[String]) {
    use rand::Rng;

    let mut count: usize = 1;
    let mut size: Option<usize> = None;
    let mut max_value: usize = 1_000_000;
    let mut min_value: usize = 1;
    let mut solvable: Option<bool> = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--count" => {
                i += 1;
                count = args[i].parse().expect("Invalid count");
            }
            "--size" => {
                i += 1;
                size = Some(args[i].parse().expect("Invalid size"));
            }
            "--max-value" => {
                i += 1;
                max_value = args[i].parse().expect("Invalid max-value");
            }
            "--min-value" => {
                i += 1;
                min_value = args[i].parse().expect("Invalid min-value");
            }
            "--solvable" => solvable = Some(true),
            "--unsolvable" => solvable = Some(false),
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let s = size.unwrap_or_else(|| rng.gen_range(1_000_000_000..=5_000_000_000usize));
        generator::generate(s, min_value, max_value, solvable);
    }
}
