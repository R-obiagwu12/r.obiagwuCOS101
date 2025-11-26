use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let _file = std::fs::File::create("ronald.txt").expect("Create has failed");
   let mut file = OpenOptions::new().append(true).open("ronald.txt").expect("Unable to open file");
   file.write_all("\nHello Class".as_bytes()).expect("Write failed");
   file.write_all("\nThis is the appendage of the document".as_bytes()).expect("Write failed");
   println!("file append is succesful");

}
