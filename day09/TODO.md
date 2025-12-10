# Sorted Line Segments

Try partitioning the input line segments into two groups: vertical and
horizontal.  Sort the vertical lines by x position, and horizontal lines
by y position.  Then we can binary search them when doing the containment
test.

For example, for vertical lines whose x is between the rectangle's left
and right, just check whether the top and bottom are both outside the
rectangle's top and bottom.  (Actually, if line.top >= rect.bottom, or
line.bottom <= rect.top.)  Similarly for horizontal lines.
