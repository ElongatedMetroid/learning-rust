fn main() {
    println!("{}", is_even(2));
    println!("{}", is_even(1));
}

/* 
   pub: function is public and can be accessed in multiple files
   fn: function definition keyword
   (num: u8): function arguments; name and type
   -> bool: will return a boolean value
*/
pub fn is_even(num: u8) -> bool{
    let digit: u8 = num % 2;
    digit == 0 /* If digit is equal to zero (zero because a positive number mod 2 is always 0) return true 
                  Also, the first statment without a semicolon ';' will be returned */
}