extern crate clap;
use clap::{Arg, App};

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
        .arg(Arg::with_name("LIMIT")
            .short("l")
            .long("limit")
            .takes_value(true)
            .help("Integer limit for the sieve to generate primes up to"))
        .arg(Arg::with_name("COUNT")
            .short("c")
            .long("count")
            .takes_value(true)
            .help("The number of primes to display"))
        .get_matches();
    let limit: i32 = (matches.value_of("LIMIT").unwrap_or("100")).parse::<i32>().unwrap();
    let count: i32 = (matches.value_of("COUNT").unwrap_or(&limit.to_string()[..])).parse::<i32>().unwrap();
    println!("Limit: {} \nCount: {}", limit, count);

    // Calculate and display N largest primes.
    let res: Vec<i32> = prime_sieve(limit);
    let count: i32 = if res.len() < count as usize { res.len() as i32 } else { count };
    let min: i32 = res.len() as i32 - count;
    let max: i32 = res.len() as i32;
    let iter: std::ops::Range<i32> = min..max;
    for n in iter {
        println!("{}", res[n as usize]);
    }
}

fn prime_sieve(limit: i32) -> Vec<i32> {
    // Generate vector containing numbers 2 through to limit.
    let mut sieve: Vec<bool> = Vec::new();
    for _ in 0..(limit+1) {
        sieve.push(true);
    }

    // Iterate over values marking a number as prime and 'sieving' out
    // multiples of that which are less than the limit.
    // TODO: Fix to work in bound range (Sqrt of limit)
    let mut primes: Vec<i32> = Vec::new();
    let _bound: i32 = (limit as f32).sqrt() as i32 + 1;
    for x in 2..limit {
        if sieve[x as usize] {
            primes.push(x);
            let mut multiple = x;
            while multiple < limit {
                sieve[multiple as usize] = false;
                multiple += x;
            }
        }
    }

    return primes;
}


