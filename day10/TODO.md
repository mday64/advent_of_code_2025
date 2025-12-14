# Part 2
## Simplex Method
### Crates
Google search recommends the `rustplex` or `simplex` crates.  It says that the
`rustplex` crate has user-friendly API.

There is also `good_lp`, which is a front end for other LP solver back ends.  Also mentioned `ellp` and `relp`.

I think I'm going to start with `simplex`.  It seems like it will be a bit easier to configure with externally parsed data, since it just takes Vec's of values.

[simplex on docs.rs](https://docs.rs/crate/simplex/latest)

Well.  The simplex crate produces a "can't continue" error for the first example
machine.

The rustplex crate has almost zero comments in the code, and only a simple
README to show how to use it.  Further, the GitHub project has one open
issue that is clearly an attempt to use the crate to solve Day 10, part 2,
with the first example machine.

### Formulating the Problem

Consider one of the machines from the example:
`[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}`

It has 4 buttons, and 6 joltages.  There will be 6 equations in 4 variables.  Let $x_i$ be the number of button presses for button $i$.

#### Goal

We want to minimize the total number of button presses: $x_0 + x_1 + x_2 + x_3$.

#### Contstraints

For each joltage, we need to sum the buttons that increment that joltage, and set it equal to the desired joltage.  In the example, consider the first joltage, the first 10.  That is joltage index 0.  Buttons 0, 1, and 2, have index 0 listed.  So: $x_0 + x_1 + x_2 = 10$

Here are all of the equations:
$$
& x_0   & +     & x_1   & +     & x_2   &       &       & = & 10  \\
& x_0   & +     &       &       & x_2   & +     & x_3   & = & 11  \\
& x_0   & +     &       &       & x_2   & +     & x_3   & = & 11  \\
& x_0   & +     & x_1   &       &       &       &       & = & 5   \\
& x_0   & +     & x_1   & +     & x_2   &       &       & = & 10  \\
&       &       &       &       & x_2   &       &       & = & 5   \\
$$
