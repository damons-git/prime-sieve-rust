# Sieve of Eratosthenes

A prime sieve implementation written in Rust.

This program will generate the list of primes up to a given limit.

## Testing
To execute the test suite for Eratosthenes prime sieve run the command:
```
$ cargo test
```

## Building
To build the application run:
```
$ cargo build
```

## Excuting the program
To execute the program, ensure the application has been built, and run the following command:
```
$ ./target/debug/prime-sieve
```

By default this prime sieve will generate and display the full list of primes up to 100. Command line arguments are present to increase this limit, as well as to filter and display only the N largest primes.

For example, the following commands both will calculate primes to 50 and output an array containing the top 3.
```
$ ./target/debug/prime-sieve --limit 50 --count 3
```
```
$ ./target/debug/prime-sieve -l 50 -c 3

```