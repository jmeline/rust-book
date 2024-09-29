fn main() {
    let temp = 100;
    let result: i32 = fahrenheit_to_celsius(temp);
    println!("{temp}F in Celsius is {result}");
}

fn fahrenheit_to_celsius(temp_f: i32) -> i32 {
    ((temp_f - 32) * 5) / 9
}
