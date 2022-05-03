fn main() {
    let crow = Bird{name: String::from("Crow"), attack: 5};
    crow.print_name();
    println!("Can {} fly? {}", crow.name, crow.can_fly());
    println!("Is {} an animal? {}", crow.name, crow.is_animal());
}

struct Bird{
    name: String,
    attack: u64
}

/* Implement functionality for the type */
impl Bird{
    fn print_name(&self){
        println!("{}", self.name);
    }
}

/* Implement animal trait for bird */
impl Animal for Bird{
    fn can_fly(&self) -> bool{
        true
    }
}

trait Animal{
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool{
        true
    }
}
