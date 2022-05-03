fn main() {
    let arr = [0, 1, 2, 3]; /* We know the length at compile time */
    let slice = &arr[1 .. 3]; /* [1, 2] We dont know the length at compile time */

    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]){
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}