# linprog

A Rust library for optimizing [linear programing](https://en.wikipedia.org/wiki/Linear_programming) (LP) models, implemented using [Dantzig's simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm).
Linprog provides utilities to create and optimize dynamic LP models.

Linprog will be available on [crates.io](https://crates.io).

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
linprog = "0.1"
```
Then include it in your code like this:
```rust
use linprog;
```

## Example (with story)
Lets say we own a company that produces three products: 
 - Product `A` selling at `50$`
 - Product `B` selling at `100$`
 - Product `C` selling at `110$`

Our company has three machienes: 
 - Machiene `X` with a maximum operating minutes per week of `2500`
 - Machiene `Y` with a maximum operating minutes per week of `2000`
 - Machiene `Z` With a maximum operating minutes per week of `450`
 

Every product needs to be processed by one of the machienes for a specific amount of time:
 - One unit of `A` needs 
   - `10` min. at X 
   - `4`  min. at Y
   - `1`   min. at Z
 - One unit of `B` needs 
   - `5`  min. at X 
   - `10` min. at Y 
   - `1.5` min. at Z
 - One unit of `C` needs 
   - `6`  min. at X 
   - `9`  min. at Y 
   - `3`   min. at Z
 

Our question is: How mutch units do we want to produce for each product in order to `maximize` our profit?

In our Rust program the data could look like this:
```rust
let price: [f64;3] = [50.0, 100.0, 110.0];
let max_workload: [f64;3] = [2500.0, 2000.0, 450.0];
let prod_machiene_time: [[f64;3];3] = [
    [10.0, 4.0, 1.0],
    [5.0, 10.0, 1.5],
    [6.0, 9.0, 3.0],
];
```
We will now construct our model (for explanation on the methods, refer to the documentation):
```rust
let mut model = Model::new("ABC_Company", Objective::Max);
let mut vars: Vec<Var> = Vec::new();
```
Then register our variables:
```rust
// Register variables 
// corresponding to the number of units produced for each product p
for p in 0..3 {
    vars.push(model.reg_var(price[p]));
}
```
Register the constraints:
```rust
// Register our constraints:
// For every machiene m: 
// sum the workload for each product p at this machiene 
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
```
Finally we solve our model and print the results:
```rust
// Solve the model
model.solve();
// Print the output
print!("The optimum is at {}$.\n", model.optimum().unwrap());
for p in 0..3 {
    print!("We need to produce {} units of product {}.\n",
        model.x(&vars[p]).unwrap(), p
    );
}
```
The output will look like this:
```
The optimum is at 22738.095238095237$.
We need to produce 178.57142857142856 units of product 0.
We need to produce 85.71428571428572 units of product 1.
We need to produce 47.61904761904763 units of product 2.
```
Make of this what you will :ok_woman: