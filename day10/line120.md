Consider line 120 of the full input:
`[.###...] (0,2,3,4,6) (0,1,3,4) (0,1,2,4,5,6) (0,2,3,5) (1,5,6) {40,182,28,34,24,186,176}`

There are 5 buttons and 7 joltages.  The minimum number of presses is 186.
The actual number of presses needed is 204.

Maximum presses for a button is the smallest joltage associated with the button.
Minimum presses is total presses minus the max presses of the other buttons.
Assuming the answer of 204 presses:

Button Min Presses Max Presses
0        0          24
1        0          24
2        0          24
3        0          28
4      104         176

----

Consider this line from the example:
`[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}`

There are 6 buttons and 4 joltages.  Answer was 10 total presses.

Button  (3)     (1,3)   (2)     (2,3)   (0,2)   (0,1)
Min     0       0       0       0       0       0
Max     7       5       4       4       3       3

Consider the second line from the example:
`[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}`

There are 6 buttons and 5 joltages.  Answer was 12.

Button  (0,2,3,4)   (2,3)   (0,4)   (0,1,2) (1,2,3,4)
Min     0           1       0       0       0
Max     2           7       2       5       2

Consider the third line from the example:
`[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}`

There are 4 buttons and 6 joltages.  Answer was 11.

Button  (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2)
Min     0           0       0           0
Max     5           5       10          11
