use std::io::Read;

fn main(){
    let file = std::fs::File::create("ronald.txt").expect("Create has failed");
    let mut file = std::fs::File::open("ronald.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
