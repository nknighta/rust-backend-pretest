use std::io;
use std::str::SplitWhitespace;

fn read_line() -> String {
    let mut line:String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn command () {
    let mut line:String = read_line();
    let mut command:SplitWhitespace = line.split_whitespace();
    if line == "a" {
        println!("OK!");
    } else {
        println!("NG!");
    }
}

fn main() {
    println!("XC RUNNER 2023");
    println!("command");
    command();
}
