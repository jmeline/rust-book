fn main() {
    println!("---------------Fahrenheit to Celsius--------------");
    let temp = 100.0;
    let result: f64 = fahrenheit_to_celsius(temp);
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

    if n == 0 {
        return n0;
    } else if n == 1 {
        return n1;
    }

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

fn fahrenheit_to_celsius(temp_f: f64) -> f64 {
    ((temp_f - 32.0) * 5.0) / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(100.0), 37.77777777777778);
        assert_eq!(fahrenheit_to_celsius(50.0), 10.0);
        assert_eq!(fahrenheit_to_celsius(25.0), -3.888888888888889);
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

