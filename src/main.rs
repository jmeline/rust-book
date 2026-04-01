pub mod exercises;

use crate::exercises::{
    hello_world,
    variables_and_mutability,
    functions
};


fn main() {
    println!("Learning rust from the book");
    hello_world::hello_world();
    variables_and_mutability::main();
    functions::main();
}
