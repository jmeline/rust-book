pub mod exercises;

use crate::exercises::{
    hello_world,
    variables_and_mutability,
    functions,
    loops,
    fahrenheit_to_celsius,
};


fn main() {
    println!("Learning rust from the book");
    hello_world::hello_world();
    variables_and_mutability::main();
    functions::main();
    loops::main();
    println!("Temperature: {f}f is {c:.00}c", f=10.0, c=fahrenheit_to_celsius::fahrenheit_to_celsius(10.0));
}
