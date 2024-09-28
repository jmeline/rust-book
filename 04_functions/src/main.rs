fn main() {
    println!("Hello, world!");
    another_function();
    basic_parameters(5, 6);
    print_labeled_mesurements(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

fn basic_parameters(x: i32, y: i32) {
    println!("The value of x is: {}, y is {}", x, y);
}

fn print_labeled_mesurements(length: i32, unit: char) {
    println!("The length is: {}{}", length, unit);
}
