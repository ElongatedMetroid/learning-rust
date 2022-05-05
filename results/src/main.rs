#[derive(Debug)]
enum Error{
    Error1
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, Error> {
    if dividend % divisor != 0 {
        Err(Error::Error1)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let div1 = divide(5, 2);
    match div1 {
        Ok(ref i) => println!("{}", i),
        Err(ref i) => println!("{:?}", i),
    }

    let div2 = divide(4, 2);
    if div2.is_ok() {
        println!("{}", div2.unwrap());
    }

    let div3 = div1.expect("bruh");    
    println!("{}", div3); /* If div3 is valid it will reach here */
}
