fn sum(a: i64, b: i64) -> i64 {
    a + b
}
fn main() {
    let a: i64 = 10;
    let b: i64 = 20;

    let s = sum(a, b);
    println!("Sum of {} and {} is {}", a, b, s);
}
