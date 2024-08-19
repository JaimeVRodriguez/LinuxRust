fn main() {
    let a: i64 = 42;
    let s = "Hello Rustaceans";
    let h: u32 = 4207849484;
    let fname = "Jaime";
    let lname = "Rodriguez";

    println!();
    println!("a == {}", a);
    println!("s == {}", s);
    println!("x == {:X}", h);
    println!("x == {:x}", h);

    println!("{0}, {1}", fname, lname);
}
