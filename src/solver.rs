use std::collections::HashMap;

#[allow(dead_code)]
fn is_optimal(tableau: &Vec<Vec<f64>>) -> bool {
    return !tableau[0][..tableau[0].len() - 1].iter().any(|&x| x > 0f64);
}

#[allow(dead_code)]
fn is_unbounded(tableau: &Vec<Vec<f64>>, pivot_column: usize) -> bool {
    return !tableau[1..].iter().any(|x| x[pivot_column] > 0f64);
}

#[allow(dead_code)]
fn optimum(tableau: &Vec<Vec<f64>>, variable_count: usize) -> (HashMap<usize, f64>, f64) {
    let mut solution: HashMap<usize, f64> = HashMap::with_capacity(variable_count);
    for column_index in 0..variable_count {
        for row in tableau[1..].iter() {
            if row[column_index] == 1f64 {
                if solution.contains_key(&column_index) {
                    solution.insert(column_index, 0f64);
                    break;
                } else {
                    solution.insert(column_index, *row.last().unwrap());
                }
            } else if row[column_index] != 0f64 {
                solution.insert(column_index, 0f64);
                break;
            }
        }
    }
    (solution, -*tableau[0].last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn tableaus() -> [Vec<Vec<f64>>; 4] {
        [
            // Not optimal
            // Unbounded in column 1
            vec![
                vec![1.0, -1.0, 0.0],
                vec![6.7, 0.0, -3.0],
                vec![0.0, -8.0, 400.68],
            ],
            // Optimal
            // Unbounded in columns 0,2
            vec![
                vec![0.0, -8.66, -5.0],
                vec![0.0, 5.46, -9.7],
                vec![-1.0, -1.0, 0.0],
            ],
            // Optimal
            // Bounded in every column
            vec![
                vec![0.0, -8.66, -22.4, -8.9, -5.0],
                vec![0.0, 1.0, 1.0, 0.0, 9.7],
                vec![1.0, 0.0, 0.0, 0.0, 4.8],
                vec![0.0, -1.0, 0.0, 1.0, 0.0],
            ],
            // Optimal
            // Unbounded in every column
            vec![
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
            ],
        ]
    }

    #[test]
    fn test_is_optimal() {
        let tableaus = tableaus();
        assert!(!is_optimal(&tableaus[0]));
        assert!(is_optimal(&tableaus[1]));
        assert!(is_optimal(&tableaus[3]));
    }

    #[test]
    fn test_is_unbounded() {
        let tableaus = tableaus();
        assert!(is_unbounded(&tableaus[1], 0));
        assert!(!is_unbounded(&tableaus[1], 1));
        assert!(is_unbounded(&tableaus[1], 2));
        assert!(is_unbounded(&tableaus[3], 0));
    }

    #[test]
    fn test_optimum() {
        let tableaus = tableaus();
        let mut solution1 = HashMap::new();
        solution1.insert(0, 4.8);
        solution1.insert(1, 0.0);
        solution1.insert(2, 9.7);
        assert_eq!((solution1, 5.0), optimum(&tableaus[2], 3));
    }
}
