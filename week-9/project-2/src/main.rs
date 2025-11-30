use std::fs::File;
use std::io::Write;

fn main() {
    // Create student data using vectors of tuples
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
        ("Adams Aliyu", "EC01010101", "Economics", "100"),
        ("Shania Bolade", "CSC10328828", "Computer", "200"),
        ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
    ];
    
    // Create and write to file
    let mut file = File::create("pau_smis_students.txt").expect("create failed");
    
    // Write header
    file.write_all("PAU SMIS - Student Records\n".as_bytes()).expect("write failed");
    file.write_all("===========================\n\n".as_bytes()).expect("write failed");
    file.write_all("Student Name        | Matric. Number | Department   | Level\n".as_bytes()).expect("write failed");
    file.write_all("--------------------|----------------|--------------|------\n".as_bytes()).expect("write failed");
    
    // Write student data with fixed spacing
    for student in &students {
        file.write_all(student.0.as_bytes()).expect("write failed");
        file.write_all("         | ".as_bytes()).expect("write failed"); // Adjust spaces as needed
        file.write_all(student.1.as_bytes()).expect("write failed");
        file.write_all("  | ".as_bytes()).expect("write failed");
        file.write_all(student.2.as_bytes()).expect("write failed");
        file.write_all("     | ".as_bytes()).expect("write failed"); // Adjust spaces as needed
        file.write_all(student.3.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
        
        // Display to console
        println!("{}         | {}  | {}     | {}", student.0, student.1, student.2, student.3);
    }
    
    println!("\nStudent records saved to 'pau_smis_students.txt'");
}