enum CoffeeSize {
    Small,
    Medium,
    Large,
}

enum MilkOption {
    None,
    Regular,
    Soy
}

struct CoffeeOrder {
    size: CoffeeSize,
    milk: MilkOption,
}

fn order_info(coffee_order: CoffeeOrder) {
    match coffee_order.size {
        CoffeeSize::Small => print!("You've ordered a Small coffee with "),
        CoffeeSize::Medium => print!("You've ordered a Medium coffee with "),
        CoffeeSize::Large => print!("You've ordered a Large coffee with"),
    }
    match coffee_order.milk {
        MilkOption::None => print!("no milk"),
        MilkOption::Regular => print!("regular milk"),
        MilkOption::Soy => print!("soy milk"),
    }
}

fn main() {

    let my_oder = CoffeeOrder {
        size: CoffeeSize::Medium,
        milk: MilkOption::Soy,
    };

    order_info(my_oder);
}
