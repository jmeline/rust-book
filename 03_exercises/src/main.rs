fn main() {
    println!("---------------Fahrenheit to Celsius--------------");
    let temp = 100;
    let result: i32 = fahrenheit_to_celsius(temp);
    println!("{temp}F in Celsius is {result}");
    println!("---------------Fibbonnaci Numbers--------------");
    println!("fib(15) = {}", fibo(12));
}


fn fibo(n: u32) -> u32 {
    let mut n0: u32 = 0;
    let mut n1: u32 = 1;
    let mut next: u32;
    // println!("n0: {n0}");
    // println!("n1: {n1}");

    let mut count = 2;
    loop {
        next = n0;
        n0 = n1;
        n1 = n1 + next;
        // println!("n{count}: {n1}");

        if count == n {
            break;
        }
        count += 1;
    }
    n1
}

fn fahrenheit_to_celsius(temp_f: i32) -> i32 {
    ((temp_f - 32) * 5) / 9
}

