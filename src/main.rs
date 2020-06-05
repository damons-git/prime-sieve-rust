extern crate clap;
use clap::{Arg, App};
mod vector_fn;

/**
 * Sieve of Eratosthenes for primality testing.
 *
 * For a number to be considered prime it must be only
 * divisible by itself and one.
 *
 * Note: The above criteria does not hold for the value 1,
 * as it is only divisible by itself.
 */

fn main() {
    let matches = App::new("Eratosthenes Prime Sieve")
        .version("0.0.1")
        .author("damons-git")
        .about("A simple application that generates the list of prime numbers within a limit.")
        .arg(Arg::with_name("COUNT")
            .short("c")
            .long("count")
            .takes_value(true)
            .help("Integer limit for the sieve to generate primes up to"))
        .get_matches();

    let count: &str = matches.value_of("count").unwrap_or("100");
    let res: Vec<i32> = prime_sieve(count.parse::<i32>().unwrap());
    println!("{:?}", res);
}

fn prime_sieve(limit: i32) -> Vec<i32> {
    // Generate vector containing numbers 2 through to limit.
    let mut sieve: Vec<i32> = Vec::new();
    for x in 2..(limit+1) {
        sieve.push(x);
    }

    // Iterate over opts 'sieving' out values.
    // If x is unmarked then it is prime
    // remove all multiples of x that are less than n (Multiples are composite)
    // All unmarked numbers remaining are prime.
    let mut invalid: Vec<i32> = Vec::new();
    let bound: i32 = (limit as f32).sqrt() as i32 + 1;
    for x in 2..bound {
        let mut multiple = x;
        while multiple < limit {
            multiple += x;
            if !invalid.contains(&multiple) {
                invalid.push(multiple);
            }
        }
    }

    let res: Vec<i32> = vector_fn::diff(sieve, invalid);
    return res;
}


