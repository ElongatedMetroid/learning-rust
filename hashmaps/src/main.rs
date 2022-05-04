use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(0, "Hello");
    map.insert(1, "Hi");
    println!("{:?}", map);

    match map.get(&0){
        Some(str) => println!("{}", str),
        None => println!("Doesnt exist in map"),
    }

    match map.get(&2){
        Some(str) => println!("{}", str),
        None => println!("Doesnt exist in map"), /* We only have elements 0 and 1 in the map, no 2 */
    }

    map.remove(&0);
    println!("{:?}", map);
}
