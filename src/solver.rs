// TODO Big-M
// TODO Revised simplex method?
// TODO faster HashMap Hashing?
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

#[allow(dead_code)]
// TODO Other pricing methods?
fn pivot(tableau: &Vec<Vec<f64>>) -> Option<(usize, usize)> {
    // TODO benchmark over iter() version
    let mut max_column: (usize, f64) = (0, -1.0 / 0.0);
    for (column_index, &value) in tableau[0][..tableau[0].len() - 1].iter().enumerate() {
        if value > max_column.1 {
            max_column = (column_index, value);
        }
    }
    if is_unbounded(tableau, max_column.0) {
        return Option::None;
    }
    let right_side_column = tableau[0].len() - 1;
    let mut min_row: (usize, f64) = (0, 1.0 / 0.0);
    for (row_index, row) in tableau[1..].iter().enumerate() {
        if row[max_column.0] > 0f64 && (row[right_side_column] / row[max_column.0]) < min_row.1 {
            min_row = (row_index, row[right_side_column] / row[max_column.0]);
        } else if row[max_column.0] > 0f64
            && (row[right_side_column] / row[max_column.0]) == min_row.1
        {
            panic!("Possible degeneracy");
        }
    }
    Option::Some((min_row.0 + 1, max_column.0))
}

#[allow(dead_code)]
// TODO Maybe avoid closures because of overhead of env. catching?
// esp. second
fn next(tableau: &mut Vec<Vec<f64>>, (pivot_row, pivot_column): (usize, usize)) {
    let pivot = tableau[pivot_row][pivot_column];
    tableau[pivot_row] = tableau[pivot_row].iter().map(|&x| x / pivot).collect();
    for row_index in 0..tableau.len() {
        if row_index != pivot_row {
            tableau[row_index] = tableau[row_index]
                .iter()
                .enumerate()
                .map(|(column, &x)| {
                    x - tableau[row_index][pivot_column] * tableau[pivot_row][column]
                })
                .collect();
        }
    }
}

#[allow(dead_code)]
pub fn solve(tableau: &mut Vec<Vec<f64>>, variable_count: Option<usize>) -> (Option<HashMap<usize, f64>>, f64) {
    let position_b = tableau[0].len() - 1;
    for row in tableau[1..].iter() {
        if row[position_b] < 0f64 {
            return solve_two_phases(tableau, position_b);
        }
    }
    let mut pivot_element: (usize, usize);
    while !is_optimal(tableau) {
        match pivot(tableau) {
            Option::Some(x) => pivot_element = x,
            Option::None => return (Option::None, 1.0 / 0.0),
        }
        next(tableau, pivot_element);
    }
    let variable_count = match variable_count {
        Option::Some(variable_count) => variable_count,
        Option::None => (position_b) - (tableau.len() - 1),
    };
    let (map, value) = optimum(tableau, variable_count);
    (Option::Some(map), value)
}

#[allow(dead_code)]
pub fn solve_two_phases(tableau: &mut Vec<Vec<f64>>, position_b: usize) -> (Option<HashMap<usize, f64>>, f64) {
    // Count #AV needed
    let mut number_artificial_variables = 0;
    for row in tableau[1..].iter() {
        if row[position_b] < 0f64 {
            number_artificial_variables += 1;
        }
    }
    // New (phase one) objective function
    // TODO maybe with_capacity
    let mut phase_one_objective_fn: Vec<f64> = vec![0f64; position_b + 1];
    // Add AV to constraints
    for (row_index, row) in tableau[1..].iter_mut().enumerate() {
        if row[position_b] < 0f64 {
            // Change +/- and build phase one objective function
            for (variable, value) in row.iter_mut().enumerate() {
                *value *= -1f64;
                phase_one_objective_fn[variable] += *value;
            }
            //*row = row.iter().map(|x| -x).collect();
            let b = row.pop().unwrap();
            for i in 0..number_artificial_variables {
                if i == row_index {
                    row.push(1f64);
                } else {
                    row.push(0f64);
                }
            }
            row.push(b);
        } else {
            let b = row.pop().unwrap();
            for _ in 0..number_artificial_variables {
                row.push(0f64);
            }
            row.push(b);
        }
    }
    // Add zeros for AV in phase one objective function
    let z = phase_one_objective_fn.pop().unwrap();
    for _ in 0..number_artificial_variables {
        phase_one_objective_fn.push(0f64);
    }
    phase_one_objective_fn.push(z);
    // Change objective functions
    let mut phase_two_objective_fn: Vec<f64> = tableau[0].to_vec();
    tableau[0] = phase_one_objective_fn;
    // Solve phase one
    let (solution, value) = solve(tableau, Option::Some(position_b));
    // Check for possible solution
    if value != 0f64 {
        panic!("Model is infeasable");
    }
    (solution, value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn tableaus() -> [Vec<Vec<f64>>; 4] {
        [
            // x* = (x1,x2) = (3.6, 0.4)
            // opt: 7.6
            vec![
                vec![2.0, 1.0, 0.0, 0.0, 0.0],
                vec![2.0, -3.0, 1.0, 0.0, 6.0],
                vec![1.0, 1.0, 0.0, 1.0, 4.0],
            ],
            // x* = (x1,x2) = (20, 17)
            // opt: 94
            vec![
                vec![3.0, 2.0, 0.0, 0.0, 0.0],
                vec![-1.0, 2.0, 1.0, 0.0, 14.0],
                vec![1.0, -1.0, 0.0, 1.0, 3.0],
            ],
            // x* = (x1,x2) = None
            // opt: 1.0/0.0
            vec![
                vec![1.0, 2.0, 0.0, 0.0, 0.0, 0.0],
                vec![-2.0, -1.0, 1.0, 0.0, 0.0, 2.0],
                vec![3.0, -4.0, 0.0, 1.0, 0.0, 12.0],
                vec![1.0, 0.0, 0.0, 0.0, 1.0, 2.0],
            ],
            // Two phase method needed
            vec![
                vec![2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                vec![1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 40.0],
                vec![-2.0, -1.0, 1.0, 0.0, 1.0, 0.0, -10.0],
                vec![0.0, 1.0, -1.0, 0.0, 0.0, 1.0, -10.0],
            ],
        ]
    }

    #[test]
    fn test_solve() {
        let mut tableaus = tableaus();
        let mut solution = HashMap::new();
        solution.insert(0, 3.6);
        solution.insert(1, 0.4);
        assert_eq!((Option::Some(solution), 7.6), solve(&mut tableaus[0], Option::None));
        let mut solution = HashMap::new();
        solution.insert(0, 20.0);
        solution.insert(1, 17.0);
        assert_eq!((Option::Some(solution), 94.0), solve(&mut tableaus[1], Option::None));
        assert_eq!((Option::None, 1.0 / 0.0), solve(&mut tableaus[2], Option::None));
    }

    #[test]
    fn test_solve_two_phases() {
        let mut tableaus = tableaus();
        let mut solution = HashMap::new();
        solution.insert(0, 10.0);
        solution.insert(1, 0.0);
        solution.insert(2, 10.0);
        solution.insert(3, 20.0);
        solution.insert(4, 0.0);
        solution.insert(5, 0.0);
        assert_eq!((Option::Some(solution), 0.0), solve(&mut tableaus[3], Option::None));
    }
}
