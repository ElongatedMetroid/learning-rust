fn main() {
    /* Types on the stack will be copyed */
    let x = 5;
    let y = x; /* Copy */

    /* Note: the 'String' type is stored on the heap, s1 holds a pointer that points to the String in the heap */
    let s1 = String::from("Hello");
    let s2 = s1; /* This is called a move, s2 is now the only way to access the String, s2 now is a pointer to String "Hello", and s1 is nothing */

    /* If we want to copy s1 to s2 you would do the following */
    /* 
        let s1 = String::from("Hello");
        let s2 = s1.clone();
    */

    /* -- Ownership With Functions -- */
    let s = String::from("Hello");
    takes_ownership(s); /* a_string now has ownership of String */
    /* Wont work, a_string has ownership
    println!("{}", s);
    */
    let i = 5;
    makes_copy(i);
    println!("{}", x);

    let str1 = gives_ownership();
    let str2 = String::from("Hello");
    let str3 = takes_and_gives_back(str2);

    let str4 = String::from("Hello World");
    let len = calculate_length(&str4); /* calculate_length takes in a reference to a string */

    let mut str5 = String::from("Hello");
    change_string(&mut str5);
    println!("{}", str5);
}

fn takes_ownership(a_string: String){
    println!("{}", a_string);
}

fn makes_copy(a_integer: i32){
    println!("{}", a_integer);
}

fn gives_ownership() -> String {
    let a_string = String::from("Hello");

    a_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

/* By default you cannot modify a reference to a String, you can fix this by using a mutable reference */
fn change_string(s: &mut String) {
    s.push_str(", world!");
}