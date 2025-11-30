use std::fs::File;
use std::io::Write;

fn main() {
    // Create and write merged data to file
    let mut file = File::create("efcc_commissioners_merged.txt").expect("create failed");
    
    // Write header
    file.write_all("EFCC - CONVICTED MINISTERS DATASET\n".as_bytes()).expect("write failed");
    file.write_all("===================================\n\n".as_bytes()).expect("write failed");
    file.write_all("S/N | NAME OF COMMISSIONER        | MINISTRY         | GEOPOLITICAL ZONE\n".as_bytes())
    .expect("write failed");
    file.write_all("----|------------------------------|------------------|-------------------\n".as_bytes())
    .expect("write failed");
    
    // Write each commissioner manually without loops
    // Commissioner 1
    file.write_all("  1 | Aigbogun Alamba Daudu       | Internal Affairs | South West\n".as_bytes())
    .expect("write failed");
    println!("  1 | Aigbogun Alamba Daudu      | Internal Affairs | South West");
    
    // Commissioner 2
    file.write_all("  2 | Murtala Afeez Bendu        | Justice          | North East\n".as_bytes())
    .expect("write failed");
    println!("  2 | Murtala Afeez Bendu        | Justice          | North East");
    
    // Commissioner 3
    file.write_all("  3 | Okorocha Calistus Ogbona   | Defense          | South South\n".as_bytes())
    .expect("write failed");
    println!("  3 | Okorocha Calistus Ogbona   | Defense          | South South");
    
    // Commissioner 4
    file.write_all("  4 | Adewale Jimoh Akanbi       | Power & Steel    | South West\n".as_bytes())
    .expect("write failed");
    println!("  4 | Adewale Jimoh Akanbi       | Power & Steel    | South West");
    
    // Commissioner 5
    file.write_all("  5 | Osazuwa Faith Etieye       | Petroleum        | South East\n".as_bytes())
    .expect("write failed");
    println!("  5 | Osazuwa Faith Etieye       | Petroleum        | South East");
    
    println!("\nMerged commissioners data saved to 'efcc_commissioners_merged.txt'");
}
