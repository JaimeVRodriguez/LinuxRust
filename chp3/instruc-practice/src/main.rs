fn dothis(a: i32) -> i32 {
    println!("Executing dothis");
    a*8
}
fn main() {
    let _a = 20;
    println!("Hello, world!");

    println!("Return value of dothis(): {}", dothis(8));
}
