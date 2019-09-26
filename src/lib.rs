mod solver;

use uuid::Uuid;

pub struct Model {
    name: String,
    state: State,
    objective: Objective,
    variables: Vec<Variable>,
    constraints: Vec<Vec<f64>>,
    tableau: Vec<Vec<f64>>,
    optimum: Option<f64>,
}

enum State {
    VariableRegistration,
    ConstraintRegistration,
    PostRegistration,
}

pub enum Objective {
    Max,
    Min,
}

struct Variable {
    uuid: Uuid,
    x: Option<f64>,
    objective_value: f64,
}

pub struct Var {
    reference: Uuid,
}

pub struct Summand(pub f64, pub Var);

pub enum Operator {
    Ge, // >=
    E,  // ==
    Le, // <=
}

impl Model {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
}
