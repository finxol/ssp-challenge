/// Subset Sum Problem solver.
/// Takes the vector size, the target sum, and the vector of values.
pub fn ssp(size: usize, target: usize, values: Vec<usize>) -> Vec<usize> {
    let mut solution = vec![false; size];

    if dfs(&values, &mut solution, target, 0, 0) {
        finalise(values, solution)
    } else {
        vec![]
    }
}

fn dfs(
    values: &[usize],
    solution: &mut Vec<bool>,
    target: usize,
    index: usize,
    current_sum: usize,
) -> bool {
    if current_sum == target {
        return true;
    }
    if index >= values.len() {
        return false;
    }

    // Left branch: include values[index]
    let new_sum = current_sum + values[index];
    if new_sum <= target {
        solution[index] = true;
        if dfs(values, solution, target, index + 1, new_sum) {
            return true;
        }
        solution[index] = false;
    }

    // Right branch: exclude values[index]
    dfs(values, solution, target, index + 1, current_sum)
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

/// Sum the values for the selected indices
fn sum(values: &Vec<usize>, targets: &Vec<bool>) -> usize {
    values
        .iter()
        .zip(targets.iter())
        .map(|(v, t)| if *t { *v } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&vec![1, 2, 3], &vec![true, false, true]), 4);
    }

    #[test]
    fn test_sum_all_selected() {
        assert_eq!(sum(&vec![5, 10, 15], &vec![true, true, true]), 30);
    }

    #[test]
    fn test_sum_none_selected() {
        assert_eq!(sum(&vec![5, 10, 15], &vec![false, false, false]), 0);
    }

    #[test]
    fn test_sum_single_element() {
        assert_eq!(sum(&vec![42], &vec![true]), 42);
    }

    #[test]
    fn test_sum_single_element_not_selected() {
        assert_eq!(sum(&vec![42], &vec![false]), 0);
    }

    #[test]
    fn test_sum_large_values() {
        assert_eq!(sum(&vec![1000, 2000, 3000], &vec![true, false, true]), 4000);
    }

    #[test]
    fn test_sum_only_last() {
        assert_eq!(
            sum(
                &vec![1, 2, 3, 4, 5],
                &vec![false, false, false, false, true]
            ),
            5
        );
    }
}
