fn main() {
    // Vector of tuples (Name, Years of Experience)
    let applicants = vec![("Ronald", 5),("Omuwa", 12),("Ozioma", 7),("Kelechi", 15),("Joshua", 3),];

    let mut most_experienced = applicants[0];

    for person in &applicants {
        if person.1 > most_experienced.1 {
            most_experienced = *person;
        }
    }

    println!("Most Experienced Candidate:");
    println!("Name: {}", most_experienced.0);
    println!("Years of Experience: {}", most_experienced.1);
}

