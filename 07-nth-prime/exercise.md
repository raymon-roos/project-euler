> The following problem is taken from Project Euler.
> https://projecteuler.net/problem=7

## Problem 7: 10001st Prime

By listing the first six prime numbers: 2,3,5,7,11, and 13, we can see that the 6th prime
is 13.

## Notes

I've solved this before for a homework exercise, so… There's a combination of simple but
significant optimizations on top of the naive solution. It was super satisfying to do
initially, because I could get massive performance gains and still understand what was
going on. Optimizations are: only checking odd numbers, only checking divisors up to the
square root of the current number, and only using previous primes as divisors for testing.
All composite (non-prime) numbers can be factored into prime numbers. Only prime numbers
cannot be factored (except into 1 and itself of course, but those aren't really factors).
So to test for primality for a given number, one only has to check known smaller prime
divisors.
