fn main() {
    for n in 0..=100 {
        if n % 3 == 0 || n % 5 == 0 {
            println!("{}", n);
        }
    }
}
