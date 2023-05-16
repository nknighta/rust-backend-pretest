use std::io;
//use db;

pub fn x (){
    intakes();
}
#[warn(unused_mut)]
fn intakes () {
    let mut x:String = read_buffer();
    println!("{}", x);
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    return buffer.trim().to_string();
}