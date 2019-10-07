//! A library for optimizing linear programming (LP) models.
//! 
//! Provides simple utilities to create and optimize dynamic LP models.
//! 
//! Currently does not support the two phase method.
// TODO custom error types instead of Result<_,&'static str>
// TODO add print method / trait on model and variable
// TODO add names for variables (see README-printing issue)
mod solver;

use uuid::Uuid;

/// Representation of a linear programming model.
pub struct Model {
    name: String,
    state: State,
    objective: Objective,
    variables: Vec<Variable>,
    constraints: Vec<Vec<f64>>,
    tableau: Vec<Vec<f64>>,
    optimum: Option<f64>,
}

#[derive(PartialEq)]
enum State {
    VariableRegistration,
    ConstraintRegistration,
    PostRegistration,
}

/// A linear programs objective.
pub enum Objective {
    /// Maximize
    Max,
    /// Minimize
    Min,
}

#[derive(PartialEq)]
struct Variable {
    uuid: Uuid,
    x: Option<f64>,
    objective_value: f64,
}

/// A representation of a variable used in the linear program.
pub struct Var {
    reference: Uuid,
}

/// A pair of factor and variable for constructing sums.
pub struct Summand<'a>(pub f64, pub &'a Var);

/// A constraints comparing operator.
pub enum Operator {
    /// Greater or equal: `>=`
    Ge,
    /// Equal: `==`
    E,
    /// Less or equal: `<=`
    Le,
}

impl Model {
    /// Creates a new `Model`.
    pub fn new(name: &str, objective: Objective) -> Self {
        Model {
            name: String::from(name),
            state: State::VariableRegistration,
            objective,
            variables: vec![],
            constraints: vec![],
            tableau: vec![],
            optimum: Option::None,
        }
    }

    /// Submits the registered variables or constraints to the `Model` and changes its phase to the next.
    /// This method can be called implicitly by calling `reg_constr` or `solve`.
    /// 
    /// The `Model`s lifetime follows three strictly seperated phases:
    /// 
    ///  - In the first phase, variables are registered.
    ///  - In the second phase, constraints are registered.
    ///  - In the third phase, the `Model` can be solved / optimized.
    /// 
    /// After the variables or constraints are submitted to the `Model` they can not be changed again (The phases can not be reverted or modified).
    pub fn update(&mut self) -> &mut Self {
        match self.state {
            State::VariableRegistration => self.state = State::ConstraintRegistration,
            State::ConstraintRegistration => self.state = State::PostRegistration,
            State::PostRegistration => (),
        }
        self
    }

    /// Registers a variable for the `Model`.
    /// # Panics
    /// This method panics if the variables were already submitted. See [`update`](#method.update).
    pub fn reg_var(&mut self, objective_value: f64) -> Var {
        if let State::VariableRegistration = self.state {
            self.variables.push(Variable {
                uuid: Uuid::new_v4(),
                x: Option::None,
                objective_value,
            });
        } else {
            panic!("Variables are already set");
        }
        Var {
            reference: self.variables.last().unwrap().uuid,
        }
    }

    /// Returns the optimal value for a given variable.
    /// # Errors
    /// This method will return an Error if the optimal value has not been calculated yet.
    pub fn x(&self, req: &Var) -> Result<f64, &'static str> {
        for variable in &self.variables {
            if variable.uuid == req.reference {
                match variable.x {
                    Some(x) => return Result::Ok(x),
                    None => return Result::Err("Model not solved yet"),
                }
            }
        }
        // TODO panic or Result::Err?
        panic!("Variable not registered for this model");
    }

    /// Registers a constraint.
    /// # Panics
    /// This method panics if the constraints were already submitted. See [`update`](#method.update).
    /// 
    /// Or if one of the variables in sum does not belong to the calling `Model`.
    pub fn reg_constr(&mut self, mut sum: Vec<Summand>, op: Operator, b: f64) -> &mut Self {
        match self.state {
            State::VariableRegistration => {
                self.update();
                return self.reg_constr(sum, op, b);
            }
            State::ConstraintRegistration => {
                let mut reference_found: bool;
                for summand in &sum {
                    reference_found = false;
                    for variable in &self.variables {
                        if summand.1.reference == variable.uuid {
                            reference_found = true;
                            break;
                        }
                    }
                    if !reference_found {
                        panic!("Variable not registered for this model");
                    }
                }
                match op {
                    Operator::Ge => {
                        for summand in &mut sum {
                            summand.0 = -summand.0;
                        }
                        self.register_standard_constraint(&sum, -b);
                    }
                    Operator::E => {
                        self.register_standard_constraint(&sum, b);
                        self.reg_constr(sum, Operator::Ge, b);
                    }
                    Operator::Le => {
                        self.register_standard_constraint(&sum, b);
                    }
                }
                self
            }
            State::PostRegistration => {
                panic!("Constraints are already set");
            }
        }
    }

    fn register_standard_constraint(&mut self, sum: &Vec<Summand>, b: f64) -> &mut Self {
        // TODO Big M
        if b < 0f64 {
            panic!("Cant find valid base. BigM not supported");
        }
        let mut a: f64;
        let mut tmp: Vec<f64> = Vec::with_capacity(self.variables.len() + 1);
        for variable in &self.variables {
            a = 0f64;
            for summand in sum {
                if summand.1.reference == variable.uuid {
                    a += summand.0;
                }
            }
            tmp.push(a);
        }
        tmp.push(b);
        self.constraints.push(tmp);
        self
    }

    fn init_tableau(&mut self) -> &mut Self {
        self.tableau.clear();
        let number_of_constraints = self.constraints.len();
        let mut tmp: Vec<f64> =
            Vec::with_capacity(self.variables.len() + number_of_constraints + 1);
        match self.objective {
            Objective::Max => self
                .variables
                .iter()
                .for_each(|variable| tmp.push(variable.objective_value)),
            Objective::Min => self
                .variables
                .iter()
                .for_each(|variable| tmp.push(-variable.objective_value)),
        }
        for _ in 0..number_of_constraints + 1 {
            tmp.push(0f64);
        }
        self.tableau.push(tmp);
        for (column, constraint) in self.constraints.iter().enumerate() {
            // TODO extend constraint and push or use tmp?
            let mut tmp: Vec<f64> =
                Vec::with_capacity(self.variables.len() + number_of_constraints + 1);
            constraint.iter().for_each(|&value| tmp.push(value));
            let b = tmp.pop().unwrap();
            for current_column in 0..number_of_constraints {
                if current_column == column {
                    tmp.push(1f64);
                } else {
                    tmp.push(0f64);
                }
            }
            tmp.push(b);
            self.tableau.push(tmp);
        }
        self
    }

    /// Solves / optimizes the `Model`.
    pub fn solve(&mut self) -> &mut Self {
        if let Option::None = self.optimum {
            match self.state {
                State::VariableRegistration => {
                    self.update();
                    self.update();
                    return self.solve();
                }
                State::ConstraintRegistration => {
                    self.update();
                    return self.solve();
                }
                State::PostRegistration => {
                    self.init_tableau();
                    let (solution, value) = solver::solve(&mut self.tableau, Option::None);
                    match self.objective {
                        Objective::Max => self.optimum = Option::Some(value),
                        Objective::Min => self.optimum = Option::Some(-value),
                    }
                    if let Option::Some(hash_map) = solution {
                        for (variable, x) in hash_map {
                            self.variables[variable].x = Option::Some(x);
                        }
                    }
                }
            }
        }
        self
    }

    /// Returns the optimal value if it was calculated already.
    /// # Errors
    /// This method will return an Error if the optimum has not been calculated yet.
    pub fn optimum(&self) -> Result<f64, &'static str> {
        self.optimum.ok_or_else(|| "Model not solved yet")
    }
}

#[cfg(test)]
mod tests {
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
    fn test_readme_example() {
        let price: [f64;3] = [50.0, 100.0, 110.0];
        let max_workload: [f64;3] = [2500.0, 2000.0, 450.0];
        let prod_machiene_time: [[f64;3];3] = [
            [10.0, 4.0, 1.0],
            [5.0, 10.0, 1.5],
            [6.0, 9.0, 3.0],
        ];

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
                sum.push(Summand (
                    prod_machiene_time[p][m],
                    &vars[p],
                ));
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
}
