fn main() {
    for i in 0..6{
        println!("{}", i);
    }

    let mut n = 0;
    while n < 4{
        println!("{}", n);
        n+=1;
        if n == 3 {
            println!("exiting");
            break;
        }
    }
}
