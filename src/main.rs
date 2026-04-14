mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let (size, target, values) = parser::parse_input(&args[1]);
    println!("Size: {}, Target: {}, Values: {:?}", size, target, values);
}
