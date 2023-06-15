// p148 지정한 텍스트 파일으 내용을 읽어 오기

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("읽어올 파일을 지정하십시오");
        return;
    }

    let filename = &args[1];

    let text = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => e.to_string(), 
    };

    let mut result: i32 = 0;
    for n in text.split_ascii_whitespace() {
        result += n.parse::<i32>().unwrap();
    }

    println!("{}", result);
}