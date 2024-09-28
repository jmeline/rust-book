use std::io;

fn main() {
    println!("Invalid Array element access example program");

    println!("Enter an array index");

    let arr = [1,2,3,4,5];
    println!("The array is: {:?}", arr);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}!!");
}
