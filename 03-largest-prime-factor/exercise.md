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
