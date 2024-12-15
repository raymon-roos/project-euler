> The following problem is taken from Project Euler.
> https://projecteuler.net/problem=5

## Problem 5: Smallest multiple

2520 is the smallest number that can be divided by each of the numbers from 1 to 10
without any remainder.

What is the smallest positive number that is evenly divisible (divisible with no
remainder) by all the numbers from 1 to 20?

## Notes

The naive solution would be counting up until finding a number `i` where `i % n == 0`,
with `n` in the range `[1,20]`. In the worst case it has to do 20 modulo operations per
iteration with a hyperbolic increase in iterations for each increase in n. This would be
unusable in most situations.

My first optimization from the absolute naive solution mentioned above, was to only look
for multiples of `n`. The "least common multiple" (lcm) of numbers `1` through `n` has to
be evenly divisible by all numbers included in that range. Since `n` is the largest of
that range, it is the best one to use to increment the loop, as whatever the correct
result is, it has to be a multiple of `n` (as well as of all the others), and `n` makes
the most progress per iteration.

Considering this logic, it would be apparent that the loop increment (step size) could be
bigger than just `n`, as all the other numbers also have to be represented. Consider
`lcm(1,2,3,4) = 12` (`3*4`), `lcm(1,2,3,4,5) = 60` (`3*4*5`), `lcm(1,2,3,4,5,6) = 60`
(`3*4*5`), `lcm(1,2,3,4,5,6,7) = 420` (`2*5*6*7`). In each of these cases, the lcm is the
"direct product" of a subset of the numbers included in the input range. But it's really
hard to express logically what numbers to include in this "direct product" generally.
(Apart from stating that when multiplying together they make up the lcm!) Numbers included
in this "direct product" can be both even and odd, and prime and composite. Sometimes the
largest number is included in this "direct product", sometimes it's not (look at the one
for `n = 6`).

At first glance, there seems to be a relation between whether numbers have multiples later
(or factors earlier?) in the range. But I couldn't find a simple rule like "include in the
direct product only the largest multiples or primes from the range". Take
`lcm(1,2,3,4,5,6,7,8,9,10) = 2520` for example. Its "direct product" is `4*7*9*10`. `9` is
the largest multiple of `3`, but `10` is the largest multiple of `2`, so this simple rule
does not explain the inclusion of `4` in the "direct product".

For now, I've given up trying to compute the lcm directly from a product of the numbers
from a filter function of the input range. If a clever approach exists, I couldn't think
of it. Instead, I decided to take the product of all the primes included in the input
range as the step size, and search incrementally for the lcm. Since the lcm of just prime
numbers has to be the product of all those prime numbers together, it follows that the lcm
of a range of numbers including several primes, has to be a multiple of the product of
those primes as well. This still takes drastically fewer attempts than the naive approach.
