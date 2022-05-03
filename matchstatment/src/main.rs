fn main() {
    let i = 2;
    match i{
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"), /* The '=' at the end of the range makes it inclusive; meaning it can be 4 */
        _ => println!("default")
    };
}
