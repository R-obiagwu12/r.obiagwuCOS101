use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let _file = std::fs::File::create("ronald.txt").expect("Create has failed");
   let mut file = OpenOptions::new().append(true).open("ronald.txt").expect("Unable to open file");
   file.write_all("\nEverywhere go semo".as_bytes()).expect("Write failed");
   file.write_all("\nI love this game".as_bytes()).expect("Write failed");
   println!("file append is succesful");

}
