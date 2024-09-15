use std::env;
use std::{io::{BufRead,BufReader}, process};
use std::fs::File;
use std::io;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut char_count = 0;
    let mut word_count = 0;
    let mut line_count = 0;

    for line in reader.lines() {
        let line = line.expect("打开文件失败");
        line_count += 1;
        char_count += line.chars().count();
        word_count += line.split_whitespace().count();
    }
    println!("{} {} {} {}",line_count,word_count,char_count,filename);
    Ok(())
}
