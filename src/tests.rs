use super::*;

#[test]
fn test_update() {
    let mut model = Model::new("Test-model", Objective::Max);
    assert!(State::VariableRegistration == model.state);
    model.update();
    assert!(State::ConstraintRegistration == model.state);
    model.update();
    assert!(State::PostRegistration == model.state);
    model.update();
    assert!(State::PostRegistration == model.state);
}

#[test]
fn test_update_indirect() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    vars.push(model.reg_var(3.0));
    assert!(State::VariableRegistration == model.state);
    vars.push(model.reg_var(3.0));
    model.reg_constr(vec![Summand(8.4, &vars[0])], Operator::Le, 6.0);
    assert!(State::ConstraintRegistration == model.state);
    model.reg_constr(vec![Summand(8.4, &vars[0])], Operator::Le, 5.0);
    model.solve();
    assert!(State::PostRegistration == model.state);
}

#[test]
fn test_reg_var_and_x() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    let mut variables: Vec<Variable> = vec![];
    assert!(variables == model.variables);
    // Add variable
    vars.push(model.reg_var(3.0));
    variables.push(Variable {
        uuid: vars.last().unwrap().reference,
        x: Option::None,
        objective_value: 3.0,
    });
    assert!(vars[0].reference == model.variables[0].uuid);
    assert!(variables == model.variables);
    // Add variable
    vars.push(model.reg_var(4.999));
    variables.push(Variable {
        uuid: vars.last().unwrap().reference,
        x: Option::None,
        objective_value: 4.999,
    });
    assert!(vars[1].reference == model.variables[1].uuid);
    assert!(vars[1].reference != model.variables[0].uuid);
    assert!(variables == model.variables);
}

#[test]
#[should_panic]
fn test_panic_reg_var() {
    let mut model = Model::new("Test-model", Objective::Max);
    model.reg_var(3.999);
    model.update();
    model.reg_var(4.989);
}

#[test]
#[should_panic]
fn test_x() {
    let mut model_0 = Model::new("Test-model 0", Objective::Max);
    let mut model_1 = Model::new("Test-model 1", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // Add variables
    vars.push(model_1.reg_var(3.0));
    vars.push(model_0.reg_var(4.0));
    // Test unsolved
    assert!(model_1.x(&vars[0]).is_err());
    model_0.x(&vars[0]).unwrap();
}

#[test]
fn test_optimum() {
    let model = Model::new("Test-model", Objective::Max);
    // Test unsolved
    assert!(model.optimum().is_err());
}

#[test]
fn test_reg_constr() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // Add variables
    vars.push(model.reg_var(3.0));
    vars.push(model.reg_var(4.0));
    // Add constraint
    model.reg_constr(
        vec![Summand(3.9, &vars[1]), Summand(8.4, &vars[0])],
        Operator::Le,
        6.0,
    );
    model.reg_constr(
        vec![Summand(3.9, &vars[1]), Summand(8.4, &vars[1])],
        Operator::Le,
        6.0,
    );
    model.init_tableau();
}

// TODO tests on init_tableau

#[test]
fn test_solve_1() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = (3.6, 0.4)
    // opt: 7.6
    // Add variables
    vars.push(model.reg_var(2.0));
    vars.push(model.reg_var(1.0));
    // Add constraints
    model.reg_constr(
        vec![Summand(2.0, &vars[0]), Summand(-3.0, &vars[1])],
        Operator::Le,
        6.0,
    );
    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(1.0, &vars[1])],
        Operator::Le,
        4.0,
    );
    model.solve();
    // Test
    assert_eq!(3.6, model.x(&vars[0]).unwrap());
    assert_eq!(0.4, model.x(&vars[1]).unwrap());
    assert_eq!(7.6, model.optimum().unwrap());
}

#[test]
fn test_solve_2() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = (20, 17)
    // opt: 94
    // Add variables
    vars.push(model.reg_var(3.0));
    vars.push(model.reg_var(2.0));
    // Add constraints
    model.reg_constr(
        vec![Summand(-1.0, &vars[0]), Summand(2.0, &vars[1])],
        Operator::Le,
        14.0,
    );
    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(-1.0, &vars[1])],
        Operator::Le,
        3.0,
    );
    model.solve();
    // Test
    assert_eq!(20.0, model.x(&vars[0]).unwrap());
    assert_eq!(17.0, model.x(&vars[1]).unwrap());
    assert_eq!(94.0, model.optimum().unwrap());
}

#[test]
fn test_solve_3() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = None
    // opt: INF
    // Add variables
    vars.push(model.reg_var(1.0));
    vars.push(model.reg_var(2.0));
    // Add constraints
    model.reg_constr(
        vec![Summand(-2.6, &vars[0]), Summand(-1.999, &vars[1])],
        Operator::Le,
        0.0,
    );
    model.reg_constr(
        vec![Summand(3.0, &vars[0]), Summand(-4.123, &vars[1])],
        Operator::Le,
        12.8,
    );
    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(0.0, &vars[1])],
        Operator::Le,
        2.0,
    );
    model.solve();
    // Test
    assert!(model.x(&vars[0]).is_err());
    assert!(model.x(&vars[1]).is_err());
    assert_eq!(1.0 / 0.0, model.optimum().unwrap());
}

#[test]
fn test_solve_4() {
    let mut model = Model::new("Test-model (two phase method)", Objective::Min);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = (2/3, 1/3)
    // opt: 5
    // Add variables
    vars.push(model.reg_var(6.0));
    vars.push(model.reg_var(3.0));
    // Add constraints
    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(1.0, &vars[1])],
        Operator::Ge,
        1.0,
    );
    model.reg_constr(
        vec![Summand(2.0, &vars[0]), Summand(-1.0, &vars[1])],
        Operator::Ge,
        1.0,
    );
    model.reg_constr(
        vec![Summand(0.0, &vars[0]), Summand(3.0, &vars[1])],
        Operator::Le,
        2.0,
    );
    model.solve();
    // Test
    assert_eq!(0.6666666666666666, model.x(&vars[0]).unwrap());
    assert_eq!(0.3333333333333333, model.x(&vars[1]).unwrap());
    assert_eq!(5.0, model.optimum().unwrap());
}

#[test]
fn test_solve_5() {
    let mut model = Model::new("Test-model (two phase method)", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = (0, 2)
    // opt: 2
    // Add variables
    vars.push(model.reg_var(0.0));
    vars.push(model.reg_var(1.0));
    // Add constraints
    model.reg_constr(
        vec![Summand(-1.0, &vars[0]), Summand(-1.0, &vars[1])],
        Operator::Le,
        -1.0,
    );
    model.reg_constr(
        vec![Summand(2.0, &vars[0]), Summand(3.0, &vars[1])],
        Operator::Le,
        6.0,
    );
    model.solve();
    // Test
    assert_eq!(0.0, model.x(&vars[0]).unwrap());
    assert_eq!(2.0, model.x(&vars[1]).unwrap());
    assert_eq!(2.0, model.optimum().unwrap());
}

#[test]
fn test_readme_example() {
    let price: [f64; 3] = [50.0, 100.0, 110.0];
    let max_workload: [f64; 3] = [2500.0, 2000.0, 450.0];
    let prod_machiene_time: [[f64; 3]; 3] = [[10.0, 4.0, 1.0], [5.0, 10.0, 1.5], [6.0, 9.0, 3.0]];

    let mut model = Model::new("ABC_Company", Objective::Max);
    let mut vars: Vec<Var> = Vec::new();
    // Register variables corresponding to the number of units produced for each product
    for p in 0..3 {
        vars.push(model.reg_var(price[p]));
    }
    // Register our constraints:
    // For every machiene m: sum the workload for each product p at this machiene
    // and make sure it stays below our maximum workload for this machiene
    for m in 0..3 {
        let mut sum: Vec<Summand> = Vec::new();
        for p in 0..3 {
            sum.push(Summand(prod_machiene_time[p][m], &vars[p]));
        }
        model.reg_constr(sum, Operator::Le, max_workload[m]);
    }
    // Solve the model
    model.solve();
    // Print the output
    /*print!("The optimum is at {}$.\n", model.optimum().unwrap());
    for p in 0..3 {
        // TODO names for variables
        print!("We need to produce {} units of product {}.\n", model.x(&vars[p]).unwrap(), p);
    }*/
    // Test
    assert_eq!(178.57142857142856, model.x(&vars[0]).unwrap());
    assert_eq!(85.71428571428572, model.x(&vars[1]).unwrap());
    assert_eq!(47.61904761904763, model.x(&vars[2]).unwrap());
    assert_eq!(22738.095238095237, model.optimum().unwrap());
}
