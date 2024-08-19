fn main() {
    let a = 42;
    let b = a;


    println!("a = {}, b = {}", a, b);

    println!("Address of a = {:p}", &a);
    println!("Address of b = {:p}", &b);

    let x = String::from("one");

    {
        let y = x;
        println!("y = {}", y);
    }
    println!("y = {}", y);
}
