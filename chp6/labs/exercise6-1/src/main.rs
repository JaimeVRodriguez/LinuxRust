enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn print_activity(day: Day) {
    match day {
        Day::Monday => println!("Monday is Rust practice day!"),
        Day::Tuesday => println!("Tuesday is gym day!"),
        Day::Wednesday => println!("Wednesday is study day!"),
        Day::Thursday => println!("Thursday is coding project day!"),
        Day::Friday => println!("Friday is movie night!"),
        Day::Saturday => println!("Saturday is relaxation day!"),
        Day::Sunday => println!("Sunday is family time!"),
    }
}

fn main() {
    let monday = Day::Monday;
    let tuesday = Day::Tuesday;
    let wednesday = Day::Wednesday;
    let thursday = Day::Thursday;
    let friday = Day::Friday;
    let saturday = Day::Saturday;
    let sunday = Day::Sunday;

    print_activity(monday);
    print_activity(wednesday);
    print_activity(sunday)
}
