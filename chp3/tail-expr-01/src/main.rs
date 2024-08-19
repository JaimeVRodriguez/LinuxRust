fn sum(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    let a = 2;
    let b = 3;

    println!("Sum of {} and {} = {}", a, b, sum(a, b));

    let x: i64 = 4;

    let odd: bool = {
        if x % 2 == 0 {
            false
        } else {
            true
        }
    };

    println!("Is {} odd? {}", x, odd);
}

