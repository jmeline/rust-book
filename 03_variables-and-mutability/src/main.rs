const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

    // variables
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
}
