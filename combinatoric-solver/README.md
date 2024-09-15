## Purpose

This crate is a purely generic combinatoric solver.

It allows to solve the problem of having n items to choose each with multiple variants and choosing the best combination of variants.

A more pratical example is the original reason for writing this solver: given a list of university course that needs to be taken with multiple groups what is the top combination of group to maximise the number of day off and hours during the morning.

## Limitation

The algorithm explores all of the solution space for the number of combination even if it efficiently cut bad solutions. The problem is still NP-hard, so a combinatorial explosion can still happen with weak bounds and a large solution space.

## Assumptions

It function on the following assumptions:

- Adding an item to the collection of item can only worsen or keep the score the same. In other words, the score function must be monotonic and decreasing.
- An invalid combination can't become valid when adding the next variant.
- The item object is really expensive to clone at lot of time, so a variant representation which is cheap to clone is used to iterate.
- Only the top N combinations are interesting.

## How it works

- It iterate recursively on all variants until it has a variant for each item.
- If an incomplete collection doesn't respect the constraint the rest of combination will not be explored.
- If an incomplete collection have a worst score than the worst of the best n scores, the rest of the combination of this collection will not be explored.
