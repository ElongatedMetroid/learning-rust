fn main() {
    /* Unsigned integer, includes u8, u16, u32, u64 */
    let unsigned: u8 = 10;

    /* Signed integer, include i8, i16, i32, i64 */
    let signed: i8 = -10;

    /* Float */
    let float: f32 = 1.23456789;

    println!("unsigned: {}, signed: {}, float {}", unsigned, signed, float);

    /* Char/String */
    let letter = "c";
    let emoji = "\u{1F600}"; /* Smile Face :D */

    println!("letter: {}, emoji: {}", letter, emoji);

    /* Boolean */
    let is_true: bool = true;

    println!("is_true: {}", is_true);

    /* Arrays   Array of unsigned 8 bit integers, size of 3*/
    let array: [u8; 3] = [1, 2, 3];
    let array2: [u8; 2] = [100, 5];

    println!("index: {}, length: {}", array[0], array.len());

    /* Print structure of array and other objects */
    println!("{:?}", array2);

    /* Tuples, like an array but can hold many types */
    /* You do not have to define the types, rust can figure this out */
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    /* OR: let tuple = (5, true, 2.1); */
    let tuple2 = (3, 5);

    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    /* Print structure of array */
    println!("{:?}", tuple2);

    let(a, b, c) = tuple;

    println!("a : first {}, b : second {}, c : third {}", a, b, c);
}
