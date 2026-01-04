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

        // Do Gaussian elimination to convert the matrix to (roughly)
        // reduced row echelon form.
        let free_columns = equations.reduce();

        // Try combinations of values for the free variables, solve for the
        // remaining variables, and pick the most optimum solution.
        // Use itertools > multi_cartesian_product.
        // Take advantage of the maximum number of presses for any given button.
        let min_presses = free_columns.iter()
            .map(|&col| 0i32..(max_presses[col]+1))
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
            .expect("No valid combination found");
        // println!("Line {}: {min_presses} presses", _machine_num+1);
        min_presses
    }).sum()
}

#[cfg(test)]
mod test {
    use super::part2;

    static EXAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    static FULL_INPUT: &str = include_str!("../../../input.txt");

    #[test]
    fn test_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 33);
    }

    #[test]
    fn test_line3() {
        let input = "[.###] (0,1) (0,3) (3) (0,2) (1,3) {14,19,7,36}\n";
        assert_eq!(part2(input), 43);
    }

    #[test]
    fn test_line22() {
        // See if this causes numeric overflow
        let input = "[#..#..#.##] (4,5,6,8,9) (0) (0,1,6,7,9) (1,2,3,4,5,8,9) (0,1,2,6,7,8,9) (0,1,7) (3,5,6,7,8) (2,4,5,6) (1,2,3,4,6,9) (0,3,6,7,8,9) (0,2,4,5,6,7,8,9) (1,3,4,5,6,8,9) {36,35,15,28,33,36,56,41,50,50}\n";
        assert_eq!(part2(input), 68);
    }

    #[test]
    fn test_line26() {
        let input = "[###.] (0) (1,2) (1,3) (0,1,3) (0,2) (0,1) {52,43,35,10}\n";
        assert_eq!(part2(input), 65);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 15631);
    }
}
