use std::io::prelude::*;
use std::fs;
use std::fs::File;

fn fs_main() {
   let text = fs::read_to_string("D:\\text.txt").unwrap();
   println!("{}", text);

   let content = fs::read("D:\\text.txt").unwrap();
    println!("{:?}", content);

    let mut buffer = [0u8; 5];
    let mut file = fs::File::open("D:\\text.txt").unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);

    fs::write("D:\\text.txt", "FROM RUST PROGRAM").unwrap();

    let mut file = File::create("D:\\text.txt").unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
}