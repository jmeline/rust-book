pub fn fibo(n: u128) -> u128 {
    let mut n0: u128 = 0;
    let mut n1: u128 = 1;
    let mut next: u128;
    // println!("n0: {n0}");
    // println!("n1: {n1}");

    if n == 0 {
        return n0;
    } else if n == 1 {
        return n1;
    }

    let mut count: u128 = 2;
    loop {
        next = n0;
        n0 = n1;
        n1 += next;
        // println!("n{count}: {n1}");

        if count == n {
            break;
        }
        count += 1;
    }
    n1
}
