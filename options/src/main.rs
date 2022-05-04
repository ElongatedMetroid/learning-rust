fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    match divide(10, 5){
        Some(ans) => println!("{}", ans),
        None => println!("Woah that is a decimal!"),
    }

    let div1 = divide(10, 2);
    let div2 = divide(2, 3);

    println!("{:?} unwraps to {}", div1, div1.unwrap());

    /* Unwraping 'None' will panic! */
    // println!("{:?} unwraps to {}", div2, div2.unwrap());
}
