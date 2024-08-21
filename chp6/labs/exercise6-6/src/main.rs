struct Employee {
    first_name: String,
    last_name: String,
    job_title: String,
}

struct Department{
    name: String,
    employees: [Employee; 5],
}
fn main() {
    let employee1 = Employee {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        job_title: String::from("Software Developer"),
    };

    let employee2 = Employee {
        first_name: String::from("Jane"),
        last_name: String::from("Smith"),
        job_title: String::from("Project Manager"),
    };

    let employee3 = Employee {
        first_name: String::from("Tom"),
        last_name: String::from("Brown"),
        job_title: String::from("Software Developer"),
    };

    let employee4 = Employee {
        first_name: String::from("David"),
        last_name: String::from("Brown"),
        job_title: String::from("HR Manager"),
    };

    let employee5 = Employee {
        first_name: String::from("Eve"),
        last_name: String::from("Davis"),
        job_title: String::from("Sales Representative"),
    };

    let department = Department {
        name: String::from("Engineering"),
        employees: [employee1, employee2, employee3, employee4, employee5],
    };

    println!("Department: {}", department.name);
    for employee in department.employees {
        println!("Employee: {} {}, Job Title: {}", employee.first_name, employee.last_name, employee.job_title);
    }
}
