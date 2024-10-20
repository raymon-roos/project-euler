> The following problem is taken from Project Euler.
> https://projecteuler.net/problem=5

## Problem 5: Smallest multiple

2520 is the smallest number that can be divided by each of the numbers from 1 to 10
without any remainder.

What is the smallest positive number that is evenly divisible (divisible with no
remainder) by all the numbers from 1 to 20?

## Notes

The naive solution would be counting up until finding a number i where i % n == 0, with
n in the range \[1;20\]. That seems rather computationally expensive to me. In the worst
case it has to do 20 modulo operations per iteration. Though in most cases it will fail
sooner.
