use nom::{IResult, Parser, branch::alt, character::complete::{char, multispace0, multispace1, newline, space0, space1, u64}, combinator::{all_consuming, opt}, multi::{many1, separated_list1}, sequence::delimited};

pub fn part1(input: &str) -> u64 {
    let (numbers, operators) = parse_input(input);

    operators.into_iter().enumerate().map(|(index, ch)| {
        let values = numbers.iter().map(|row| row[index]);
        match ch {
            '+' => values.into_iter().sum::<u64>(),
            '*' => values.into_iter().product(),
            _ => unreachable!()
        }
    }).sum()
}

//
// In this part, numbers are top-to-bottom in a single column.
// The operator appears on the last line, in the first column of a problem.
// The operators line has trailing blank spaces to match the last column of digits.
// Numbers in a given column may be top-aligned or bottom-aligned, so just ignore
// those spaces.  We know we've finished a problem when there are no digits in
// a column.
//
pub fn part2(input: &str) -> u64 {
    let mut lines: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    // Add a blank column on the end since that triggers us to do the
    // addition or subtraction.  (It's a terminator for the values.)
    for line in lines.iter_mut() {
        line.push(b' ');
    }

    let operators = lines.pop().unwrap();

    let mut operator = b'+';     // Will be overridden by first column
    let mut result = 0;         // Overall function result
    let mut values: Vec<u64> = Vec::new();

    for (col, op) in operators.into_iter().enumerate() {
        if op != b' ' {
            operator = op;
        }

        // Gather digits from this column
        let digits: Vec<u64> = lines.iter().filter_map(|line| {
            let ch = line[col];
            if ch == b' ' {
                None
            } else {
                Some((ch - b'0') as u64)
            }
        }).collect();

        if digits.len() == 0 {
            // Do the math on the gathered values
            if operator == b'+' {
                result += values.iter().sum::<u64>();
            } else {
                result += values.iter().product::<u64>();
            }
            values.clear();
        } else {
            // Push this value
            let value = digits.into_iter().fold(0, |acc, digit| acc * 10 + digit);
            values.push(value);
        }
    }

    result
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let (_, (numbers, operators)) = all_consuming(
        (many1(number_row), operator_row)
    ).parse(input).expect("Invalid input");

    (numbers, operators)
}

fn number_row(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        space0,
        separated_list1(space1, u64),
        (opt(space0), newline)
    ).parse(input)
}

fn operator_row(input: &str) -> IResult<&str, Vec<char>> {
    delimited(
        space0,
        separated_list1(multispace1, alt((char('*'), char('+')))),
        opt(multispace0)
    ).parse(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    
    static EXAMPLE_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    static FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 4277556);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 4878670269096);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 3263827);
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(part2(FULL_INPUT), 8674740488592);
    }
}
