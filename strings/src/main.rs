fn main() {
    /* Pointer to a place in memory that holds "Hello World" */
    let str: &str = "Hello World";
    /* Create variable string that is equal to "Hello World" with a type of String ( A literal String ) */
    let mut string: String = String::from("Hello World");

    let slice = &string[0 .. 6]; /* Get everything from index 0 to index 6 */
    slice.len();

    /* Push a character onto the string */
    string.push('!');
    string.push_str("\nGoodbye World!");
    string = string.replace("World", "Human");

    println!("{}", string);
}
