fn main() {
    simple_if_else(3);
    simple_if_else(8);
    for n in 1..=11 {
        check_divisibility(n);
    }

    println!("-----------------loops-----------------");
    loops_example();
    loops_example2();
    while_example();
    while_through_collection();
}

fn simple_if_else(number:i32) {
    if number < 5 {
        println!("condition was true {number} < 5");
    } else {
        println!("Condition was false {number} >= 5");
    }
}

fn check_divisibility(number: i32) {
    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} is not divisible by 4, 3, or 2");
    }
}

fn loops_example() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 23;
        }
    };
    println!("The result is {}", result);

}

fn loops_example2() {
    let mut count = 0;
    'counting: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_example() {
    let mut number = 5;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn while_through_collection() {
    let a = [10, 20, 30, 40, 50];

    // slow way
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    for element in a {
        println!("the value is: {}", element);
    }

    for element in [10, 20, 30, 40, 50] {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}
