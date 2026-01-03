use day10::parsing::parse_input;
mod matrix;
use matrix::Matrix;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../../input.txt");
    let result = part2(input);
    println!("Part 2 (Gaussian): {result}")
}

fn part2(input: &str) -> i32 {
    let (_, machines) = parse_input(input).expect("Invalid input");
    machines.into_iter().enumerate().map(|(_machine_num, machine)| {
        // println!("Machine #{_machine_num}");
        let mut free_columns = Vec::<usize>::new();
        let num_rows = machine.joltages.len();
        let num_buttons = machine.buttons.len();

        // Figure out the maximum number of presses for each button
        let max_presses: Vec<i32> = machine.buttons.iter().map(|button| {
            button.iter().map(|&j| machine.joltages[j as usize]).min().unwrap() as i32
        }).collect();

        // Create a system of linear equations based on which buttons
        // map to which joltage.
        let mut equations = Matrix::new(num_rows, num_buttons+1);
        for (b, button) in machine.buttons.iter().enumerate() {
            for &j in button.iter() {
                equations[j as usize][b] = 1;
            }
        }
        for (j, joltage) in machine.joltages.iter().enumerate() {
            equations[j][num_buttons] = *joltage as i32;
        }
        // equations.print();

        // Do Gaussian elimination (or similar) to come up with a
        // solution (or family of solutions).  Start with row
        // echelon form.
        // equations.print();
        let mut arranged_rows = 0;
        for col in 0..num_buttons {
            let mut found_pivot = false;
            // println!("Checking column {col}");

            // Find a row below `arranged_rows` with a non-zero entry in column `col`.
            for row in arranged_rows..num_rows {
                if equations[row][col] != 0 {
                    // println!("Found non-zero in row {row}");
                    if row > arranged_rows {
                        // Move row `row` to arranged_rows + 1
                        // println!("Swapping rows {row} and {}", arranged_rows);
                        equations.swap(row, arranged_rows);
                        // equations.print();
                    }
                    arranged_rows += 1;
                    
                    // If any rows below `arranged_rows`` have a non-zero entry in row `col`,
                    // use row subtraction to change that column to zero.
                    // let pivot = equations[row][col];
                    for row in arranged_rows..num_rows {
                        if equations[row][col] != 0 {
                            equations.subtract_rows(arranged_rows - 1, row, col);
                            // equations.print();
                        }
                    }

                    // We are done with the current column.
                    found_pivot = true;
                    break;
                }
            }
            if !found_pivot {
                free_columns.push(col);
            }
        }

        // Try to get close to reduced row echelon form.  For each column
        // that is uniquely determined, substitute into rows above.
        // println!("Starting Reduced Row Echelon Form");
        for row in (0..num_rows).rev() {
            // println!("Checking row {row}");
            // Find the first non-zero column in this row
            for col in 0..num_buttons {
                if equations[row][col] != 0 {
                    // println!("Found pivot in column {col}");
                    // TODO: Should we try to divide out any common factor for this row?
                    // Eliminate non-zero values above this entry
                    for r in 0..row {
                        if equations[r][col] != 0 {
                            equations.subtract_rows(row, r, col);
                            // equations.print();
                        }
                    }
                    break;
                }
            }
        }

        // Remove rows of all zeroes
        equations.rows.retain(|row| row.iter().any(|&v| v != 0));
        // equations.print();

        // Try combinations of values for the free variables, solve for the
        // remaining variables, and pick the most optimum solution.
        // Use itertools > multi_cartesian_product.
        // Take advantage of the maximum number of presses for any given button.
        free_columns.iter()
            .map(|&col| 0i32..max_presses[col])
            .multi_cartesian_product()
            .filter_map(|free_presses| {
                // Use the selected combination for the free variables
                let mut vars = vec![0i32; num_buttons];
                for (&col, &presses) in free_columns.iter().zip(free_presses.iter()) {
                    vars[col] = presses;
                }

                // Back substitute the non-free variables.  If a variable would
                // be negative or not an integer, then reject this solution.
                for row in equations.rows.iter().rev() {
                    // Find the first non-zero column; that's the variable we
                    // are solving for in this iteration.
                    let leading = row.iter().position(|&v| v != 0).unwrap();
                    let coefficient = row[leading];

                    // Compute the (scaled) value of this variable
                    let mut scaled = *row.last().unwrap();
                    for other_var in (leading+1)..(row.len()-1) {
                        scaled -= row[other_var] * vars[other_var];
                    }

                    // If the variable would not be an integer, or would be
                    // negative, reject this solution.
                    if scaled % coefficient != 0 || scaled / coefficient < 0 {
                        return None;
                    }

                    vars[leading] = scaled / coefficient;
                }
                Some(vars.into_iter().sum::<i32>())
            })
            .min()
            .expect("No valid combination found")
    }).sum()
}

#[cfg(test)]
static EXAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../../../input.txt");

#[test]
fn test_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 33);
}

#[test]
fn test_line22() {
    // See if this causes numeric overflow
    let input = "[#..#..#.##] (4,5,6,8,9) (0) (0,1,6,7,9) (1,2,3,4,5,8,9) (0,1,2,6,7,8,9) (0,1,7) (3,5,6,7,8) (2,4,5,6) (1,2,3,4,6,9) (0,3,6,7,8,9) (0,2,4,5,6,7,8,9) (1,3,4,5,6,8,9) {36,35,15,28,33,36,56,41,50,50}\n";
    assert_eq!(part2(input), 1);    // This is not the correct answer
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 15631);
}
