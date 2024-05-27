mod prime_generator;

use prime_generator::PrimeGenerator;
use std::time::Instant;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Prime Generator")
        .version("1.0")
        .about("Generates prime numbers")
        .subcommand(Command::new("next")
            .about("Finds the next prime number from a given start")
            .arg(Arg::new("start")
                .help("The start number to find the next prime")
                .required(true)
                .index(1)))
        .subcommand(Command::new("start")
            .about("Continuously find primes starting from a given number")
            .arg(Arg::new("from")
                .help("Start from a given number")
                .required(false))
            )
        .get_matches();

    match matches.subcommand() {
        Some(("next", sub_m)) => {
            let start = sub_m.get_one::<String>("start").unwrap().parse::<usize>().unwrap();
            let mut prime_gen = PrimeGenerator::from(start);
            if let Some(prime) = prime_gen.next() {
                println!("Next prime: {}", prime);
            }
        }
        Some(("start", sub_m)) => {
            let default_start = usize::MAX / 2;
            let from = sub_m.get_one::<String>("from")
                            .map(|s| s.parse().unwrap())
                            .unwrap_or(default_start);
            let prime_gen = PrimeGenerator::from(from);
            let mut count = 0;
            let mut previous_time = Instant::now();
            for prime in prime_gen {
                count += 1;
                let current_time = Instant::now();
                let interval = current_time.duration_since(previous_time).as_millis();
                previous_time = Instant::now();
                println!("prime: {:x} count: {} interval: {} ms", prime, count, interval);
            }
        }
        _ => {
            println!("Use either 'start' or 'run' subcommand");
        }
    }
}
