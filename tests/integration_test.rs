use linprog::*;
use std::collections::HashMap;

#[test]
fn solve_1() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = (3.6, 0.4)
    // opt: 7.6
    vars.push(model.reg_var(2.0));
    vars.push(model.reg_var(1.0));
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
    model.optimize();
    // Test
    assert_eq!(3.6, model.x(&vars[0]).unwrap());
    assert_eq!(0.4, model.x(&vars[1]).unwrap());
    assert_eq!(7.6, model.optimum().unwrap());
}

#[test]
fn solve_2() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = (20, 17)
    // opt: 94
    vars.push(model.reg_var(3.0));
    vars.push(model.reg_var(2.0));
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
    model.optimize();
    // Test
    assert_eq!(20.0, model.x(&vars[0]).unwrap());
    assert_eq!(17.0, model.x(&vars[1]).unwrap());
    assert_eq!(94.0, model.optimum().unwrap());
}

#[test]
fn solve_3() {
    let mut model = Model::new("Test-model", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = None
    // opt: INF
    vars.push(model.reg_var(1.0));
    vars.push(model.reg_var(2.0));
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
    model.optimize();
    // Test
    assert!(model.x(&vars[0]).is_err());
    assert!(model.x(&vars[1]).is_err());
    assert_eq!(1.0 / 0.0, model.optimum().unwrap());
}

#[test]
fn solve_4() {
    let mut model = Model::new("Test-model (two phase method)", Objective::Min);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = (2/3, 1/3)
    // opt: 5
    vars.push(model.reg_var_with_name(6.0, "testvar"));
    vars.push(model.reg_var_with_name(3.0, "another testvar"));
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
    model.optimize();
    // Test
    assert_eq!(0.6666666666666666, model.x(&vars[0]).unwrap());
    assert_eq!(0.3333333333333333, model.x(&vars[1]).unwrap());
    assert_eq!(5.0, model.optimum().unwrap());
}

#[test]
fn solve_5() {
    let mut model = Model::new("Test-model (two phase method)", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = (0, 2)
    // opt: 2
    vars.push(model.reg_var(0.0));
    vars.push(model.reg_var(1.0));
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
    model.optimize();
    // Test
    assert_eq!(0.0, model.x(&vars[0]).unwrap());
    assert_eq!(2.0, model.x(&vars[1]).unwrap());
    assert_eq!(2.0, model.optimum().unwrap());
}

#[test]
fn readme_example() {
    let mut model = Model::new("Readme example", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    // x* = (x1,x2) = (130, 20)
    // opt: 490
    vars.push(model.reg_var(3.0));
    vars.push(model.reg_var(5.0));
    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(2.0, &vars[1])],
        Operator::Le,
        170.0,
    );
    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(1.0, &vars[1])],
        Operator::Le,
        150.0,
    );
    model.reg_constr(
        vec![Summand(0.0, &vars[0]), Summand(3.0, &vars[1])],
        Operator::Le,
        180.0,
    );
    model.optimize();
    //print!("{}", model);
    // Test
    assert_eq!(130.0, model.x(&vars[0]).unwrap());
    assert_eq!(20.0, model.x(&vars[1]).unwrap());
    assert_eq!(490.0, model.optimum().unwrap());
}

#[test]
fn readme_example_story() {
    let products: HashMap<&str, f64> = [
        ("Product A", 50.0),
        ("Product B", 100.0),
        ("Product C", 110.0),
    ]
    .iter()
    .cloned()
    .collect();

    let machines: HashMap<&str, f64> = [
        ("Machine X", 2500.0),
        ("Machine Y", 2000.0),
        ("Machine Z", 450.0),
    ]
    .iter()
    .cloned()
    .collect();

    let mut time_needed: HashMap<(&str, &str), f64> = HashMap::new();
    time_needed.insert(("Product A", "Machine X"), 10.0);
    time_needed.insert(("Product A", "Machine Y"), 4.0);
    time_needed.insert(("Product A", "Machine Z"), 1.0);

    time_needed.insert(("Product B", "Machine X"), 5.0);
    time_needed.insert(("Product B", "Machine Y"), 10.0);
    time_needed.insert(("Product B", "Machine Z"), 1.5);

    time_needed.insert(("Product C", "Machine X"), 6.0);
    time_needed.insert(("Product C", "Machine Y"), 9.0);
    time_needed.insert(("Product C", "Machine Z"), 3.0);

    let mut model = Model::new("ABC_Company", Objective::Max);
    let mut vars: HashMap<&str, Var> = HashMap::new();

    for (product, &price) in &products {
        vars.insert(product, model.reg_var_with_name(price, product));
    }

    for (&machine, &max_time) in &machines {
        let mut sum: Vec<Summand> = Vec::new();
        for (&product, _) in &products {
            sum.push(Summand(time_needed[&(product, machine)], &vars[product]));
        }
        model.reg_constr(sum, Operator::Le, max_time);
    }

    model.optimize();

    //print!("{}", model);

    //println!("\nThe optimum is at {:.*}$.", 2, model.optimum().unwrap());
    //for (product, var) in &vars {
    //    println!("We need to produce {} units of product {}.", model.x(&var).unwrap().floor(), product);
    //}

    // Test
    assert_eq!(178.57142857142856, model.x(&vars["Product A"]).unwrap());
    assert_eq!(85.71428571428572, model.x(&vars["Product B"]).unwrap());
    assert_eq!(47.61904761904763, model.x(&vars["Product C"]).unwrap());
    assert_eq!(22738.095238095237, model.optimum().unwrap());
}
