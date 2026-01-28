use std::io::Read;

fn open_file(filename: &str) {
    let mut file = std::fs::File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    println!("{}", contents);
}

fn main() {
    let mut role = String::new();

    println!(
        "Enter your role..
1) Administrator
2) Project Manager
3) Employee
4) Customer
5) Vendor
0) Exit

Select a number from 1 - 5:"
    );

    std::io::stdin()
        .read_line(&mut role)
        .expect("Failed to read input");

    let role: i32 = role.trim().parse().expect("Please enter a number");

   match role {
    1 => open_file("C:\\Users\\NEW USER\\OneDrive\\Documents\\globacom_db.sql"),
    2 => open_file("C:\\Users\\NEW USER\\OneDrive\\Documents\\project_tb.sql"),
    3 => open_file("C:\\Users\\NEW USER\\OneDrive\\Documents\\staff_tb.sql"),
    4 => open_file("C:\\Users\\NEW USER\\OneDrive\\Documents\\customer_tb.sql"),
    5 => open_file("C:\\Users\\NEW USER\\OneDrive\\Documents\\dataplan_tb.sql"),
    0 => println!("Exiting program..."),
    _ => println!("Invalid option"),
}
}
