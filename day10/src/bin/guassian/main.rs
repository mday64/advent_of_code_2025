use day10::parsing::parse_input;
mod matrix;
use matrix::Matrix;

fn main() {
    let input = include_str!("../../../input.txt");
    let result = part2(input);
    println!("Part 2 (Gaussian): {result}")
}

fn part2(input: &str) -> i32 {
    let (_, machines) = parse_input(input).expect("Invalid input");
    machines.into_iter().map(|machine| {
        let num_rows = machine.joltages.len();
        let num_buttons = machine.buttons.len();

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

        // Do Gaussian elimination (or similar) to come up with a
        // solution (or family of solutions).  Start with row
        // echelon form.
        equations.print();
        let mut arranged_rows = 0;
        for col in 0..num_buttons {
            println!("Checking column {col}");

            // Find a row below `arranged_rows` with a non-zero entry in column `col`.
            for row in arranged_rows..num_rows {
                if equations[row][col] != 0 {
                    println!("Found non-zero in row {row}");
                    if row > arranged_rows {
                        // Move row `row` to arranged_rows + 1
                        println!("Swapping rows {row} and {}", arranged_rows);
                        equations.swap(row, arranged_rows);
                        equations.print();
                    }
                    arranged_rows += 1;
                    
                    // If any rows below `arranged_rows`` have a non-zero entry in row `col`,
                    // use row subtraction to change that column to zero.
                    // let pivot = equations[row][col];
                    for row in arranged_rows..num_rows {
                        if equations[row][col] != 0 {
                            equations.subtract_rows(arranged_rows - 1, row, col);
                            equations.print();
                        }
                    }

                    // We are done with the current column.
                    break;
                }
            }
        }

        // Try to get close to reduced row echelon form.  For each column
        // that is uniquely determined, substitute into rows above.
        println!("Starting Reduced Row Echelon Form");
        for row in (0..num_rows).rev() {
            println!("Checking row {row}");
            // Find the first non-zero column in this row
            for col in 0..num_buttons {
                if equations[row][col] != 0 {
                    println!("Found pivot in column {col}");
                    // Eliminate non-zero values above this entry
                    for r in 0..row {
                        if equations[r][col] != 0 {
                            equations.subtract_rows(row, r, col);
                            equations.print();
                        }
                    }
                    break;
                }
            }
        }

        println!("----");

        1
    }).sum()
}

#[cfg(test)]
static EXAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[test]
fn test_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 33);
}