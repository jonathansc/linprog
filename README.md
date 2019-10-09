# linprog

A Rust library for optimizing [linear programming](https://en.wikipedia.org/wiki/Linear_programming) (LP) models, implemented using [Dantzig's simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm).
Linprog provides utilities to create and optimize dynamic LP models.

This library does not (yet :turtle:) support mixed integer programming.

Linprog will be available on [crates.io](https://crates.io) soon.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
linprog = "0.3"
```
Then include it in your code like this:
```rust
use linprog;
```

## Example (with story)
Lets say a company produces three products: 
 - Product `A` selling at `50$`
 - Product `B` selling at `100$`
 - Product `C` selling at `110$`

The company has three machines: 
 - Machine `X` with a maximum operating minutes per week of `2500`
 - Machine `Y` with a maximum operating minutes per week of `2000`
 - Machine `Z` With a maximum operating minutes per week of `450`
 

Every product needs to be processed by one of the machines for a specific amount of time:
 - One unit of `A` needs 
   - `10`&nbsp;&nbsp;&nbsp;min. at `X`
   - `4`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;min. at `Y`
   - `1`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;min. at `Z`
 - One unit of `B` needs 
   - `5`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;min. at `X`
   - `10`&nbsp;&nbsp;&nbsp;min. at `Y`
   - `1.5`&nbsp;min. at `Z`
 - One unit of `C` needs 
   - `6`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;min. at `X`
   - `9`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;min. at `Y`
   - `3`&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;min. at `Z`
 

The question is: How mutch units does the company want to produce for each product in order to `maximize` their profit?

In the Rust program, the data could look like this:
```rust
let products: HashMap<&str, f64> = [
    ("Product A", 50.0),
    ("Product B", 100.0),
    ("Product C", 110.0),
].iter().cloned().collect();

let machines: HashMap<&str, f64> = [
    ("Machine X", 2500.0),
    ("Machine Y", 2000.0),
    ("Machine Z", 450.0),
].iter().cloned().collect();

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
```
A less verbose way to define the data could look like this:
```rust
let product_price: [f64;3] = [50.0, 100.0, 110.0];
let machine_max_workload: [f64;3] = [2500.0, 2000.0, 450.0];
let prod_machine_time_needed: [[f64;3];3] = [
    [10.0, 4.0, 1.0],
    [5.0, 10.0, 1.5],
    [6.0, 9.0, 3.0],
];
```
For the sake of this example, I will use the previous of the two versions.

Now onto the Model's construction:
```rust
let mut model = Model::new("ABC_Company", Objective::Max);
let mut vars: HashMap<&str, Var> = HashMap::new();
```
Then register the variables with names and prices:
```rust
for (product, &price) in &products {
    vars.insert(product, model.reg_var_with_name(price, product));
}
```
Register the constraints:
```rust
for (&machine, &max_time) in &machines {
    let mut sum: Vec<Summand> = Vec::new();
    for (&product, _) in &products {
        sum.push(Summand(time_needed[&(product, machine)], &vars[product]));
    }
    model.reg_constr(sum, Operator::Le, max_time);
}
```
Finally the model gets solved and the results get printed:
```rust
model.solve();
print!("{}", model);
```
The output will look like this:
```
Model "ABC_Company" [solved]:
    Optimum: 22738.095238095237
    Variable "Product C": 47.61904761904763
    Variable "Product A": 178.57142857142856
    Variable "Product B": 85.71428571428572
```
A customized display of the solution could be done in this way:
```rust
println!("\nThe optimum is at {:.*}$.", 2, model.optimum().unwrap());
for (product, var) in &vars {
    println!("We need to produce {} units of product {}.", model.x(&var).unwrap().floor(), product);
}
```
Leading to the following output:
```
The optimum is at 22738.10$.
We need to produce 85 units of product Product B.
We need to produce 178 units of product Product A.
We need to produce 47 units of product Product C.
```
Make of this what you want :ok_woman: