//! A library for optimizing linear programming (LP) models.
//!
//! Provides simple utilities to create and optimize dynamic LP models.
// TODO custom error types instead of Result<_,&'static str>
// TODO impl Display for Var
// Problem: variable fields not copyable / "borrow occures twice"

#[cfg(test)]
mod tests;

mod solver;

use std::fmt;
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
    name: Option<String>,
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
    // TODO simple overloading not possible?
    pub fn reg_var(&mut self, objective_value: f64) -> Var {
        self.reg_var_overload(objective_value, Option::None)
    }

    /// Registers a variable, with a given name, for the `Model`.
    /// # Panics
    /// This method panics if the variables were already submitted. See [`update`](#method.update).
    pub fn reg_var_with_name(&mut self, objective_value: f64, name: &str) -> Var {
        self.reg_var_overload(objective_value, Option::Some(String::from(name)))
    }

    fn reg_var_overload(&mut self, objective_value: f64, name: Option<String>) -> Var {
        if let State::VariableRegistration = self.state {
            self.variables.push(Variable {
                name,
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
    /// # Panics
    /// This method panics if the model is infeasable or might be degenerate
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

// I know this part does not look good
// TODO simpe solution
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.optimum {
            Option::Some(optimum) => {
                let mut i = 0;
                writeln!(
                    f,
                    "\nModel \"{}\" [solved]:\n\tOptimum: {}{}",
                    self.name,
                    optimum,
                    self.variables.iter().fold(String::new(), |acc, variable| {
                        i += 1;
                        acc + "\n\tVariable \""
                            + &variable.name.as_ref().unwrap_or(&i.to_string())
                            + "\": "
                            + &variable.x.unwrap().to_string()
                    })
                )
            }
            Option::None => writeln!(f, "\nModel \"{}\" [not solved]", self.name),
        }
    }
}
