use std::fs;

fn main() {
    let _file = std::fs::File::create("ronald.txt").expect("Create has failed");
    fs::remove_file("ronald.txt").expect("could not remove file");
    println!("File is removed");
}