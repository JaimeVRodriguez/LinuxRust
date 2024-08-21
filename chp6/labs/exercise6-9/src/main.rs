fn main() {
    // let mut nums = Vec::new();
    // for i in 0..=100 {
    //     nums.push(i);
    // }
    //
    // for mut i in nums {
    //     if i % 2 == 0 {
    //         println!("Element {} -> {}", i, i * i);
    //     } else {
    //         println!("Element {} -> {}",i, i * 2);
    //
    //     }
    // }

    let mut numbers: Vec<i64> = (0..=100).collect();

    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            numbers[i] = numbers[i] * numbers[i];
        } else {
            numbers[i] = numbers[i] * 2;
        }
    }

    for (index, value) in numbers.iter().enumerate() {
        println!("Element {} -> {}", index, value);
    }

}
