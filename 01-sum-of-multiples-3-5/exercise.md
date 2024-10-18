> The following problem is taken from Project Euler.
> https://projecteuler.net/problem=1

## Problem 1: Multiples of 3 or 5

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5,
6, and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.

## Notes

This is fizzbuzz without the strings.

**Follow-up**

I decided to create an account for project euler after all, and it turns
out you can check your answers! And it also turns out I'm stupid. My
original code didn't work, because it would count products of both 3 and
5 twice. Rookie mistake…

I also got a little side tracked running micro benchmarks against
different formulations of the solution, but I settled on my original
version with the above-mentioned fix.

If not for double-counting, you could have different loops with a step
size of 3 and 5 (or whatever dynamic multiples you choose as arguments).
But that will require an additional step to filter out duplicates, so
at that point it's not really faster any more.
