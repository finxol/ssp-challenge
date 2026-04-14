pub fn ssp(size: usize, target: usize, values: Vec<usize>) -> Vec<usize> {
    let mut solution = vec![false; size];

    for i in 0..solution.len() {
        if sum(&values, &solution) == target {
            return values
                .iter()
                .zip(solution.iter())
                .filter(|(_, t)| **t)
                .map(|(v, _)| *v)
                .collect();
        }
    }

    vec![]
}

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
