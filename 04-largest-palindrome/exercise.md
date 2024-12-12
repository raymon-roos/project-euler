> The following problem is taken from Project Euler.
> https://projecteuler.net/problem=4

## Problem 4: Largest palindrome product

A palindromic number reads the same both ways. The largest palindrome made from the
product of two 2-digit numbers is 9009 = 91 · 99.

Find the largest palindrome made from the product of two 3-digit numbers.

## Notes

I have no idea what the clever approach for this is going to be, but I really want to try
to get something working before looking up spoilers online. I'll probably want to start
searching from the top. But it doesn't make sense to just `i-=1;` all the way down, since
we're looking for symmetry in how the number is written. So for every step in the loop,
I'll want to regard the current number as split down the middle and operate on a digit in
opposite position in both halves. So decrement the first digit of the first half or the
last digit of the second half.

I suspect the clever solution to this problem might have something to do with the fact
that decimal is a [positional number system](https://en.wikipedia.org/wiki/Positional_numeral_system).

The largest product of two 3-digit numbers is 999 · 999 = 998001, and the smallest is
100 · 100 = 10000. So the input domain for finding the largest palindrome is
\[10000,998001\].

_ps:_

Well, my solution ended up nothing like what I described above. I had to contend with the
fact that any discovered palindromes have to be products of two actual n-digit numbers.
The largest palindrome counting down from 998001 is 997799 = 11 · 90709. So instead I'm
working from the factors between 100 and 999 directly and decrementing those.

Initially I had the answer wrong, because I wrote a nested loop that would return early on
the first palindrome which would be 995 · 583 = 580085, which isn't the largest palindrome
at all but merely the first you find. So I had to add a clause for tracking the largest
palindrome and let the whole nested loop run until conclusion. Another optimization would
be to break the inner loop upon finding a palindrome (break, not return from the
function).

Then I wanted to refactor the loops into an iterator chain. Interestingly enough it seemed
to produce the same results but take about 20x the time. So either this particular case is
a really poor use of iterators that the compiler can't optimize as well, or more likely
there's a semantic difference between the two that I haven't understood. A little
troubling, since I came up with both versions…

_pps:_

Some awesome people on Discord found what the difference was. The loops had an early
`p > max` clause, which the iterator chain didn't have in the same place. The current
implementation with the reverse->map->find chain was also their idea.
