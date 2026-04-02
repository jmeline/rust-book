pub fn main() {
    println!("---------------Fahrenheit to Celsius--------------");
    let temp = 100.0;
    let result: f64 = fahrenheit_to_celsius(temp);
    println!("{temp}F in Celsius is {result}");
    println!("---------------Fibbonnaci Numbers--------------");
    println!("fib(15) = {}", fibo(12));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(100.0), 37.77777777777778);
        assert_eq!(fahrenheit_to_celsius(50.0), 10.0);
        assert_eq!(fahrenheit_to_celsius(25.0), -3.888888888888889);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn test_fibo() {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(2), 1);
        assert_eq!(fibo(3), 2);
        assert_eq!(fibo(4), 3);
        assert_eq!(fibo(5), 5);
        assert_eq!(fibo(6), 8);
        assert_eq!(fibo(7), 13);
        assert_eq!(fibo(8), 21);
        assert_eq!(fibo(9), 34);
        assert_eq!(fibo(10), 55);
        assert_eq!(fibo(11), 89);
        assert_eq!(fibo(12), 144);
        assert_eq!(fibo(13), 233);
        assert_eq!(fibo(14), 377);
        assert_eq!(fibo(15), 610);
    }
}

