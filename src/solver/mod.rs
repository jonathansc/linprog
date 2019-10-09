// TODO Revised simplex method?

#[cfg(test)]
mod tests;

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
pub fn solve(
    tableau: &mut Vec<Vec<f64>>,
    variable_count: Option<usize>,
) -> (Option<HashMap<usize, f64>>, f64) {
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
fn solve_two_phases(
    tableau: &mut Vec<Vec<f64>>,
    position_b: usize,
) -> (Option<HashMap<usize, f64>>, f64) {
    // Count #AV needed
    let mut number_artificial_variables = 0;
    for row in tableau[1..].iter() {
        if row[position_b] < 0f64 {
            number_artificial_variables += 1;
        }
    }
    // Phase one
    let phase_two_objective_function =
        prepare_phase_one(tableau, number_artificial_variables, position_b);
    let (_, value) = solve(tableau, Option::Some(position_b));
    // Check if model is feasable
    if value != 0f64 {
        panic!("Model is infeasable");
    }
    // Phase two
    prepare_phase_two(
        tableau,
        phase_two_objective_function,
        number_artificial_variables,
    );
    let (solution, value) = solve(tableau, Option::None);
    (solution, value)
}

#[allow(dead_code)]
fn prepare_phase_one(
    tableau: &mut Vec<Vec<f64>>,
    number_artificial_variables: usize,
    position_b: usize,
) -> Vec<f64> {
    let mut phase_one_objective_function: Vec<f64> = vec![0f64; position_b + 1];
    // Add AV to constraints
    for (row_index, row) in tableau[1..].iter_mut().enumerate() {
        if row[position_b] < 0f64 {
            // Change +/- and build phase one objective function
            for (variable, value) in row.iter_mut().enumerate() {
                *value *= -1f64;
                phase_one_objective_function[variable] += *value;
            }
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
    let z = phase_one_objective_function.pop().unwrap();
    for _ in 0..number_artificial_variables {
        phase_one_objective_function.push(0f64);
    }
    phase_one_objective_function.push(z);
    let phase_two_objective_function: Vec<f64> = tableau[0].to_vec();
    tableau[0] = phase_one_objective_function;
    phase_two_objective_function
}

#[allow(dead_code)]
fn prepare_phase_two(
    tableau: &mut Vec<Vec<f64>>,
    mut phase_two_objective_function: Vec<f64>,
    number_artificial_variables: usize,
) {
    // Calculate phase two objective function
    let last_index = phase_two_objective_function.len() - 1;
    for variable in 0..phase_two_objective_function.len() {
        if phase_two_objective_function[variable] != 0f64 && is_base_variable(tableau, variable) {
            // Variable should be displayed by non base variables
            for row in tableau[1..].iter() {
                if row[variable] == 1f64 {
                    for column_index in 0..phase_two_objective_function.len() - 1 {
                        if column_index != variable {
                            phase_two_objective_function[column_index] +=
                                phase_two_objective_function[variable] * (-row[column_index]);
                        }
                    }
                    phase_two_objective_function[last_index] +=
                        phase_two_objective_function[variable] * row[row.len() - 1];
                }
            }
            phase_two_objective_function[variable] = 0f64;
        }
    }
    phase_two_objective_function[last_index] *= -1f64;
    tableau[0] = phase_two_objective_function;
    // Remove AVs
    for row in tableau[1..].iter_mut() {
        let b = row.pop().unwrap();
        for _ in 0..number_artificial_variables {
            row.pop();
        }
        row.push(b);
    }
}

#[allow(dead_code)]
fn is_base_variable(tableau: &Vec<Vec<f64>>, variable: usize) -> bool {
    let mut found_one = false;
    let mut ret = false;
    for row in tableau[1..].iter() {
        if row[variable] == 1f64 {
            if found_one == true {
                ret = false;
                break;
            } else {
                found_one = true;
                ret = true;
            }
        } else if row[variable] != 0f64 {
            ret = false;
            break;
        }
    }
    ret
}
