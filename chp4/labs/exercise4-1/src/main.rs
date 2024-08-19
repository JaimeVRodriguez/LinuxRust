fn main() {
    let text1:String = "some text".to_string();
    let text2 = text1;

    // println!("text1 = {}, text2 = {}", text1, text2);

    let text3 = text2.clone();
    println!("text2 = {}, text3 = {}", text2, text3);

    let mut text4 = "Jaime ".to_string();
    text4.push_str("Rodriguez");
    println!("text4 = {}", text4);}
