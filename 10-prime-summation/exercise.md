> The following problem is taken from Project Euler.
> https://projecteuler.net/problem=10

## Problem 10: Summation of Primes

The sum of the primes below 10 is `2 + 3 + 5 + 7 = 17`

Find the sum of all the primes below two million.

## Notes

Primes again. I've redone the same trick pretty often now. I feel this is very similar to
the earlier "Nth prime" problem. I suspect the crucial difference is in the kind of
optimizations available. Finding the Nth prime for a very large N has some pretty advanced
math involved I think. In this case we have to sum all the primes up to N, so either it's
intended to guide us to that one optimization I'm aware of, of using previous primes to
find new primes. Or it's designed to disallow some other optimization I'm not aware of to
skip early primes. 
