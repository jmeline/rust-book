const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

    // variables
    println!("------------------3.1 variables and mutability----------------");
    println!("------------------variables----------------");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {x}");

    println!("------------------consts----------------");
    // consts
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    println!("------------------shadowing----------------");
    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    let spaces = format!("{spaces}**********").len();
    println!("The value of spaces is: {spaces}");


    println!("------------------3.2 data types----------------");
    let _guess: u32 = "42".parse().expect("Not a number!");

    //floating point types
    let x = 2.0; // f64
    let y: f32 = 3.0; //f32
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    numberic_operations();

    println!("------------------3.2 boolean type----------------");
    let t = true;
    let f: bool = false;
    println!("The value of t is: {t}");
    println!("The value of f is: {f}");

    println!("------------------3.2 character type----------------");
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    let japanese = '日';
    println!("The value of c is: {c}, z is: {z}, heart_eyed_cat is: {heart_eyed_cat}, japanese is: {japanese}");

    println!("------------------3.2 compound types----------------");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of x is: {0} y is: {1} z is: {2}", tup.0, tup.1, tup.2);

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;
}

fn numberic_operations() {
    println!("------------------3.2 numberic operations----------------");
    // addition
    let sum = 5 + 10;
    println!("The value of sum 5 + 10 = {sum}");

    //subtraction
    let sub = 95.5 - 4.3;
    println!("The value of sub 95.5 - 4.3 = {sub}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product 4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of 56.7 / 32.2 = {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("The value of -5 / 3 = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of 43 % 5 = {remainder}");
}
