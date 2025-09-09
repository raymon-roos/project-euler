> The following problem is taken from Project Euler.
> https://projecteuler.net/problem=9

## Problem 9: Special Pythagorean triplet

A Pythagorean triplet is a set of three natural numbers, `a < b < c`, for which, 
`a^2 + b^2 = c^2`. 

For example, `3^2 + 4^2 = 5^2` 
             `9   + 16  = 25` 

There exists exactly one Pythagorean triplet for which `a + b + c = 1000`.
Find the product `abc`.

## Notes

Hey, Pythagorean triangles! Those are popular examples for list expressions in books about
Haskell. Wikipedia mentions _primitive Pythagorean triples_, the smallest of which is the
one given in the example: `(3, 4, 5)`. 
I initially did it the boring way with nested for loops. I wanted a solution using
iterators, but I needed GPTs help with that. Though I generally prefer iterators and
function chains, in this case it looks a lot worse. Interestingly enough, it also seems to
perform considerably worse, going by my primitive benchmark method.

Playing around with the numbers on paper, I realized that the result is a multiple of
a Primitive Pythagorean triple `(8, 15, 17)` - a Pythagorean triple of numbers without
common divisors apart from one (co-prime). I read all this in the first two paragraphs of
the Wikipedia page. `(3, 4, 5)` is one of these. Non-primitive triples are made by
multiplying a primitive one `(k*3, k*4, k*5)`. Or in this case, `(25*8, 25*15, 25*17)`.
But unless there is a more efficient way to generate Primitive Pythagorean triples, this
fact doesn't help with finding the solution faster. More thinking is needed.
