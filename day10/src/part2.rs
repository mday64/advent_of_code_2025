use crate::Machine;

pub fn configure_joltages(machine: &Machine) -> u32 {
    // Sort the buttons from fewest to most joltages affected.
    // This works with the way we iterate over combinations to initially
    // generate more presses on buttons that affect many joltages at once.
    let mut buttons = machine.buttons.clone();
    buttons.sort_unstable_by_key(|button| button.len());

    // Initialize the best answer to an upper bound
    let mut best: u32 = machine.joltages.iter().sum();

    // Explore various combinations of button presses.
    // Initial state is no button presses, and joltages at their maximum.
    dfs(0, &machine.joltages, &buttons, &mut best, 0);

    best
}

//
// Explore possible combinations of pressing buttons.  We don't need
// to explore a single button press at a time.  What we'll do is try
// to reduce the lowest joltage to zero, exploring all combinations
// of button presses that affect that minimal joltage.  Since buttons
// may affect more than one joltage, other joltages may be reduced, too.
//
// If we encounter a valid solution (all joltages are zero), and it has
// fewer presses than the current best, then increment the best.
//
fn dfs(presses: u32, joltages: &[u32], buttons: &[Vec<u32>], best: &mut u32, depth: u32) {
    // If we can't possibly find a better solution, then skip this branch.
    if presses + joltages.iter().max().unwrap() >= *best {
        return;
    }

    // Pick the smallest remaining joltage.  If they are all zero, then
    // we have found a solution; compare it to the best solution so far.
    let min_joltage = joltages
        .iter()
        .enumerate()
        .filter(|&(_index, &joltage)| joltage > 0)
        .min_by_key(|&(_index, joltage)| joltage);
    if min_joltage.is_none() {
        // There were no non-zero joltages, which means we have a solution.
        // See if it is better than the best discovered so far.  Either way,
        // there is nothing more in this branch to explore.
        if presses < *best {
            *best = presses;
        }
        return;
    }
    let (joltage_index, &min_joltage) = min_joltage.unwrap();

    // Find the buttons that affect that joltage, and only affect non-zero
    // joltages.  These are the buttons we can press.
    let buttons_to_press = buttons.iter()
        .filter(|button| {
            button.contains(&(joltage_index as u32)) &&
            button.iter().all(|&i| joltages[i as usize] > 0)
        })
        .cloned()
        .collect::<Vec<_>>();
    let num_buttons = buttons_to_press.len();
    
    // If there are no buttons we can press, then we're done with this branch.
    if num_buttons == 0 {
        return;
    }

    // If there is only one button we can press, then avoid the combinations
    // logic below.
    if num_buttons == 1 {
        let mut new_joltages = Vec::from(joltages);
        for &j_index in &buttons_to_press[0] {
            new_joltages[j_index as usize] -= min_joltage;
        }
        dfs(presses + min_joltage, &new_joltages, buttons, best, depth+1);
        return;
    }

    // Generate all possible ways to distribute min_joltage presses amongst
    // buttons_to_press.  They are generated in lexicographic order.
    let mut new_joltages = Vec::with_capacity(joltages.len());
    let mut combination = vec![0; num_buttons];
    combination[num_buttons - 1] = min_joltage;
    loop {
        // Compute the adjusted joltages based on the combination of presses
        new_joltages.clear();
        new_joltages.extend_from_slice(joltages);
        for (&press, button) in combination.iter().zip(buttons_to_press.iter()) {
            for &j_index in button.iter() {
                new_joltages[j_index as usize] -= press;
            }
        }

        dfs(presses + min_joltage, &new_joltages, buttons, best, depth+1);

        // Produce the next combination, or break if there are no more.
        if combination[0] == min_joltage {
            break;
        }

        // Find the rightmost non-zero count.  It is at maximum and needs to
        // wrap around to zero.  Carry one into the count to its left, and
        // add the rest to the least significant (rightmost) counter.
        //
        // NOTE: This works even if the rightmost non-zero count is the last
        // item in the vector (it will end up decrementing by 1).
        let right = combination.iter().rposition(|&v| v > 0).unwrap();
        let remainder = combination[right] - 1;
        combination[right] = 0;
        combination[right - 1] += 1;
        combination[num_buttons - 1] = remainder;
    }
}

#[cfg(test)]
mod part2_tests {
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