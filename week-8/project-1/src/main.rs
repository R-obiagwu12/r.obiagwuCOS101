use std::io;
use std::ops::Range;

fn main() {

    // Vector of professions
    let professions = vec!["lawyer","teacher","academic","office administrator",];

    // Vector storing APS rules:
    // (profession, years_range, aps_level)
    let rules = vec![
        // Lawyer
        ("lawyer", 0..3, "Paralegal (APS 1-2)"),
        ("lawyer", 3..6, "Junior Associate (APS 3-5)"),
        ("lawyer", 5..9, "Associate (APS 5-8)"),
        ("lawyer", 8..11, "Senior Associate 1-2 (EL1)"),
        ("lawyer", 10..14, "Senior Associate 3-4 (EL2)"),
        ("lawyer", 14..51, "Partner (SES)"),

        // Teacher
        ("teacher", 0..3, "Classroom Teacher (APS 1-2)"),
        ("teacher", 3..6, "Senior Teacher (APS 3-5)"),
        ("teacher", 5..9, "Leading Teacher (APS 5-8)"),
        ("teacher", 8..11, "EL1 Teacher"),
        ("teacher", 10..14, "Deputy Principal (EL2)"),
        ("teacher", 14..51, "Principal (SES)"),

        // Academic
        ("academic", 0..3, "Intern – Placement (APS 1-2)"),
        ("academic", 3..6, "Research Assistant (APS 3-5)"),
        ("academic", 5..9, "PhD Candidate (APS 5-8)"),
        ("academic", 8..11, "Post-Doc Researcher (EL1)"),
        ("academic", 10..14, "Senior Lecturer (EL2)"),
        ("academic", 14..50, "Dean (SES)"),

        // Office Administrator
        ("office administrator", 0..3, "APS 1-2"),
        ("office administrator", 3..6, "APS 3-5"),
        ("office administrator", 5..9, "Senior Administrator (APS 5-8)"),
        ("office administrator", 8..11, "Office Manager (EL1)"),
        ("office administrator", 10..14, "Director (EL2)"),
        ("office administrator", 14..50, "SES (CEO)"),
    ];

    // ----------------------------
    // GET USER INPUT
    // ----------------------------

    println!("Enter staff profession:");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession = profession.trim().to_lowercase();

    // Validate profession
    if !professions.contains(&profession.as_str()) {
        println!("Invalid profession entered.");
        return;
    }

    println!("Enter years of experience:");
    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Failed to read input");
    let years: i32 = years.trim().parse().expect("Enter a valid number");

    // ----------------------------
    // FIND APS LEVEL USING VECTOR RULES
    // ----------------------------
    let mut aps_result = "No APS level found";

    for rule in &rules {
        let (rule_prof, range, aps) = rule;

        if &profession == rule_prof && range.contains(&years) {
            aps_result = aps;
            break;
        }
    }

    println!("\nAPS Level: {}", aps_result);
}
