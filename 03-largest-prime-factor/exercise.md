> The following problem is taken from Project Euler.
> https://projecteuler.net/problem=3

## Problem 3: Largest prime factor

The prime factors of 13195 are 5, 7, 13, and 29.
What is the largest prime factor of the number 60085414755143?

## Notes

Ooh prime numbers, I've done something with those before! Unfortunately, my first approach
is a bust. I'm calculating all the prime numbers that can fit into given number n, and
then finding which of those primes are factors of n, and finally finding the largest
of those factors. This works for small numbers, but not for n ≈ 10^13.

### Take 2

After looking online I found out it makes more sense to first find all the factors of
given number n, and then finding the largest prime number among those. As extra
optimizations I'm only looking for factors up to the square root, going from high to low.
However, without calculating all the prime numbers in sequence, I can no longer use
a prime sieve. So testing for primality is more expensive this way. But by only
considering factors of n up to the square root of n, the search space for primes has
become a lot smaller.

I absolutely love how Rust's language features let me express this so succinctly. Ranges
and method chains turn the whole thing into basically two lines of code.
