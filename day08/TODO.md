# Both parts at once

Solve both parts in a single function.  Only do the parsing and connecting of points once.  We get the answer for part 1 after 1,000 connections, and the answer for part 2 once we only have one connected component left.

# Heap instead of sorting

Instead of sorting the pairs of points up front, would it be more efficient
to use a min-heap?

Yes, apparently so.  With the full input, we have 499,500 pairs.  For
part 1, we only need the first 1,000.  For part 2, we only use 3,778.

# Union-find

This is apparently a case of a union-find algorithm.  Investigate how that algorithm works, and either find a crate that implements it, or implement it myself.

