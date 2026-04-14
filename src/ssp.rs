/// Subset Sum Problem solver.
/// Takes the vector size, the target sum, and the vector of values.
pub fn ssp(size: usize, target: usize, values: Vec<usize>) -> Vec<usize> {
    let mut solution = vec![false; size];
    let mut current_sum: usize = 0;
    let mut i: usize = 0;

    loop {
        if current_sum == target {
            return finalise(values, solution);
        }

        if i < size {
            // Try including values[index]
            if current_sum + values[i] <= target {
                solution[i] = true;
                current_sum += values[i];
            }
            i += 1;
            continue;
        }

        // Backtrack: find last included value and switch to exclude
        loop {
            if i == 0 {
                return vec![];
            }
            i -= 1;
            if solution[i] {
                solution[i] = false;
                current_sum -= values[i];
                i += 1;
                break;
            }
        }
    }
}

/// Format the vector to only keep the values for the solution
fn finalise(values: Vec<usize>, solution: Vec<bool>) -> Vec<usize> {
    values
        .iter()
        .zip(solution.iter())
        .filter(|(_, t)| **t)
        .map(|(v, _)| *v)
        .collect()
}
