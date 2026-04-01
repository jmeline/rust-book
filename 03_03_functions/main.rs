fn main() {
    println!("Hello, world!");
    another_function();
    basic_parameters(5, 6);
    print_labeled_mesurements(5, 'h', "Wazzup");
    statement_and_expression();
    functions_with_return();
}

fn another_function() {
    println!("Another function.");
}

fn basic_parameters(x: i32, y: i32) {
    println!("The value of x is: {}, y is {}", x, y);
}

fn print_labeled_mesurements(length: i32, unit: char, string: &str) {
    println!("The length is: {}{} with string {}", length, unit, string);
}

fn statement_and_expression() {
    let _x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn functions_with_return() {
    fn five() -> u32 { 5 }
    fn plus_one(x: u32) -> u32 { x + 1 }

    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}
