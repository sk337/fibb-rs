use cargo_metadata::MetadataCommand;
use num_bigint::BigInt;
use std::env;
use std::time::Instant;

fn fibonacci(n: u64) -> BigInt {
    if n <= 1 {
        return BigInt::from(n);
    }

    let (mut prev, mut curr): (BigInt, BigInt) = (BigInt::from(0), BigInt::from(1));

    for _ in 2..=n {
        let next = prev.clone() + &curr;
        prev = curr;
        curr = next;
    }

    curr
}

fn print_help(authors: &[String]) {
    println!("Usage:");
    println!("  fibb-rs [OPTIONS] <FIB_NUMBER>");
    println!("");
    println!("Options:");
    println!("  -h, --help    Display this help message");
    println!("  --json        Output result in JSON format. Time is in nanoseconds");
    println!("");
    println!("Authors:");
    for author in authors {
        println!("  {}", author);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args.iter().any(|arg| arg == "-h" || arg == "--help") {
        let metadata = MetadataCommand::new().exec().unwrap();
        let authors: Vec<String> = metadata
            .workspace_members
            .iter()
            .flat_map(|id| {
                metadata
                    .packages
                    .iter()
                    .find(|p| &p.id == id)
                    .and_then(|p| Some(p.authors.clone()))
            })
            .flatten()
            .collect();

        print_help(&authors);
        return;
    }

    let mut n: u64 = 100;
    let mut output_json = false;

    for i in 1..args.len() {
        match args[i].as_str() {
            "--json" => output_json = true,
            _ => {
                if let Ok(num) = args[i].parse() {
                    n = num;
                } else {
                    println!("Invalid argument: {}", args[i]);
                    return;
                }
            }
        }
    }

    let start = Instant::now();
    let result = fibonacci(n);
    let elapsed = start.elapsed();

    if output_json {
        println!(
            "{{\"testTime\": {}, \"val\": {}, \"input\": {}}}",
            elapsed.as_nanos(),
            result,
            n
        );
    } else {
        println!("{}\n{:?}", result, elapsed);
    }
}
