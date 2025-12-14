use crate::Machine;

pub fn configure_joltages(machine: &Machine) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{Machine, part2::configure_joltages};

    #[test]
    fn part2_example_0() {
        let machine = Machine{
            indicators: vec![],
            buttons: vec![vec![3], vec![1,3], vec![2], vec![2,3], vec![0,2], vec![0,1]],
            joltages: vec![3,5,4,7],
        };

        assert_eq!(configure_joltages(&machine), 10);
    }

    #[test]
    fn part2_example_1() {
        let machine = Machine{
            indicators: vec![],
            buttons: vec![vec![0,2,3,4], vec![2,3], vec![0,4], vec![0,1,2], vec![1,2,3,4]],
            joltages: vec![7,5,12,7,2],
        };

        assert_eq!(configure_joltages(&machine), 12);
    }

    #[test]
    fn part2_example_2() {
        let machine = Machine{
            indicators: vec![],
            buttons: vec![vec![0,1,2,3,4], vec![0,3,4], vec![0,1,2,4,5], vec![1,2]],
            joltages: vec![10,11,11,5,10,5],
        };

        assert_eq!(configure_joltages(&machine), 11);
    }
}