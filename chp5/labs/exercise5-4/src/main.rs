use std::io;

fn main() {
    println!("Enter a number : ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let my_nr: i32 = guess.trim().parse().unwrap();

    println!("You entered: {}", my_nr);

    if my_nr == 0 {
        println!("You entered zero");
    } else if my_nr % 2 == 0 {
        println!("You entered an even number");
    } else {
        println!("You entered an odd number");
    }
}
