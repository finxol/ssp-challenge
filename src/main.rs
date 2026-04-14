mod parser;
mod ssp;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let (size, target, values) = parser::parse_input(&args[1]);
    println!("Size: {}, Target: {}", size, target);

    let start = std::time::Instant::now();
    let result = ssp::ssp(size, target, values);
    let elapsed = start.elapsed();

    println!("");

    if result.is_empty() {
        println!("No solution found");
    } else {
        println!("Found solution!");
        // println!("Result: {:?}", result);
    }

    println!("");
    println!("Time: {:.3?}", elapsed);
}
