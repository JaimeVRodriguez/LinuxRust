struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}
fn main() {
    let person = Person {
        first_name: String::from("Jaime"),
        last_name: String::from("Rodriguez"),
        age: 37,
    };
    println!("First name: {}, Last name: {}, Age: {}",
             person.first_name, person.last_name, person.age);
}
