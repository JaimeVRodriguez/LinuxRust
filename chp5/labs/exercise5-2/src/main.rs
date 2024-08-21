fn main() {
    let alphabet = "abcedfghijklmnopqrstuvwxyz";

    for letter in alphabet.chars() {
        if "aeiou".contains(letter) {
            println!("{} is a vowel", letter);
        }
    }
}
