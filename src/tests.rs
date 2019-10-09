// TODO tests on init_tableau
use crate::*;

#[test]
fn update() {
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
fn update_indirect() {
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
fn reg_var_and_x() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    let mut variables: Vec<Variable> = vec![];
    assert!(variables == model.variables);
    // Add variable
    vars.push(model.reg_var(3.0));
    variables.push(Variable {
        name: Option::None,
        uuid: vars.last().unwrap().reference,
        x: Option::None,
        objective_value: 3.0,
    });
    assert!(vars[0].reference == model.variables[0].uuid);
    assert!(variables == model.variables);
    // Add variable
    vars.push(model.reg_var(4.999));
    variables.push(Variable {
        name: Option::None,
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
fn panic_reg_var() {
    let mut model = Model::new("Test-model", Objective::Max);
    model.reg_var(3.999);
    model.update();
    model.reg_var(4.989);
}

#[test]
#[should_panic]
fn x() {
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
fn optimum() {
    let model = Model::new("Test-model", Objective::Max);
    // Test unsolved
    assert!(model.optimum().is_err());
}

#[test]
fn reg_constr() {
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
