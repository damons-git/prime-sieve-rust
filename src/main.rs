extern crate clap;
use clap::{Arg, App};
use std::time::Instant;

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

    // Calculate and display N largest primes.
    let start_t = Instant::now();
    let primes: Vec<i32> = prime_sieve(limit);
    let elapsed = start_t.elapsed();
    let n_primes = fetch_n_primes(count, primes);
    println!("Calculate to: {}", limit);
    println!("Display top: {}", count);
    println!("Time taken: {:?}", elapsed);
    println!("{:?}", n_primes);
}

fn fetch_n_primes(n: i32, primes: Vec<i32>) -> Vec<i32> {
    if n < 0 { panic!("Cannot fetch a negative number of results.") }

    let mut output: Vec<i32> = vec![];
    let count: i32 = if primes.len() < n as usize { primes.len() as i32 } else { n };
    let min: i32 = primes.len() as i32 - count;
    let max: i32 = primes.len() as i32;

    let iter: std::ops::Range<i32> = min..max;
    for n in iter {
        output.push(primes[n as usize]);
    }

    return output;
}

fn prime_sieve(limit: i32) -> Vec<i32> {
    if limit < 0 { panic!("Cannot have a negative limit to calculate primes up to.") }

    // Generate vector containing numbers 2 through to limit.
    let mut sieve: Vec<bool> = Vec::new();
    for _ in 0..(limit+1) {
        sieve.push(true);
    }

    // Iterate over values marking a number as prime and 'sieving' out
    // multiples of that which are less than the limit.
    let mut primes: Vec<i32> = Vec::new();
    let bound: i32 = (limit as f32).sqrt() as i32 + 1;
    for x in 2..bound {
        if sieve[x as usize] {
            primes.push(x);
            let mut multiple = x;
            while multiple < limit {
                sieve[multiple as usize] = false;
                multiple += x;
            }
        }
    }

    // Mark all integers not eliminated through composition as prime.
    for x in bound..limit {
        if sieve[x as usize] {
            primes.push(x);
        }
    }

    return primes;
}

// Module unit tests.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prime_sieve_short() {
        let res: Vec<i32> = prime_sieve(10);
        let expected: Vec<i32> = vec![2, 3, 5, 7];
        assert_eq!(res, expected);
    }

    #[test]
    fn prime_sieve_long() {
        let res: Vec<i32> = prime_sieve(100);
        let expected: Vec<i32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        assert_eq!(res, expected);
    }

    #[test]
    fn prime_sieve_edge_case() {
        let res_one: Vec<i32> = prime_sieve(1);
        let res_zero: Vec<i32> = prime_sieve(0);
        let expected: Vec<i32> = vec![];
        assert_eq!(res_one, expected);
        assert_eq!(res_zero, expected);
    }

    #[test]
    #[should_panic]
    fn prime_sieve_errorneous() {
        let res: Vec<i32> = prime_sieve(-1);
        let expected: Vec<i32> = vec![];
        assert_eq!(res, expected);
    }

    #[test]
    fn fetch_n_primes_top_three() {
        let res: Vec<i32> = fetch_n_primes(3, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);
        let expected: Vec<i32> = vec![23, 29, 31];
        assert_eq!(res, expected);
    }

    #[test]
    fn fetch_n_primes_overflow() {
        let res: Vec<i32> = fetch_n_primes(10, vec![2, 3, 5]);
        let expected: Vec<i32> = vec![2, 3, 5];
        assert_eq!(res, expected);
    }

    #[test]
    fn fetch_n_primes_edge_case() {
        let res: Vec<i32> = fetch_n_primes(0, vec![2, 3, 5, 7, 11]);
        let expected: Vec<i32> = vec![];
        assert_eq!(res, expected);
    }

    #[test]
    #[should_panic]
    fn fetch_n_primes_erroneous() {
        let _: Vec<i32> = fetch_n_primes(-1, vec![2, 3, 5]);
    }
}

