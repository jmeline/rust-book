pub mod lib;

use crate::lib::functions;

fn main() {
    println!("Fibo of 100 is {}", functions::fibo(100));
}
