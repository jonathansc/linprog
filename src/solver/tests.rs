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
fn solve_1() {
    let mut tableaus = tableaus();
    let mut solution = HashMap::new();
    solution.insert(0, 3.6);
    solution.insert(1, 0.4);
    assert_eq!(
        (Option::Some(solution), 7.6),
        optimize(&mut tableaus[0], Option::None)
    );
    let mut solution = HashMap::new();
    solution.insert(0, 20.0);
    solution.insert(1, 17.0);
    assert_eq!(
        (Option::Some(solution), 94.0),
        optimize(&mut tableaus[1], Option::None)
    );
    assert_eq!(
        (Option::None, 1.0 / 0.0),
        optimize(&mut tableaus[2], Option::None)
    );
}

#[test]
fn solve_two_phases() {
    let mut tableaus = tableaus();
    let mut solution = HashMap::new();
    solution.insert(0, 10.0);
    solution.insert(1, 10.0);
    solution.insert(2, 20.0);
    assert_eq!(
        (Option::Some(solution), 70.0),
        optimize(&mut tableaus[3], Option::None)
    );
}
