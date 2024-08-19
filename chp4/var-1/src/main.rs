fn main() {
    // Variables can not be re-assigned by default they are immutable
    // let a = 42;
    //
    // println!("address of a: {:p}", &a);
    // println!("a = {}", a);
    //
    // let a = 44;
    // println!("address of a: {:p}", &a);

    // let a = 42;
    // println!("address of a: {:p}", &a);
    // println!("a = {}", a);
    //
    // let a = 44;
    // println!("address of a: {:p}", &a);

    let mut a = 42;
    println!("address of a: {:p}", &a);
    println!("a = {}", a);

    a = 44;
    println!("address of a: {:p}", &a);
    println!("a = {}", a);
}
