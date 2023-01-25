use std::env;
use std::fs;
use std::io::Write;

pub fn find_name() {
    if env::args().len() < 4 {
        println!("Need both filename and name to search");
        return;
    }
    let filename = env::args().nth(2).unwrap();
    let name = env::args().nth(3).unwrap();
    println!("filename is {filename} and name is {name}");
    let moonwalkers = fs::read_to_string(&filename).unwrap();
    for line in moonwalkers.lines() {
        if line.cmp(&name).is_eq() {
            println!("Name found");
            return;
        }
    }
    println!("Name not found, so adding to the end of the file");
    let mut file_to_write = fs::OpenOptions::new().append(true).open(&filename).unwrap();
    file_to_write.write(b"\n").expect("Cannot write to file");
    file_to_write
        .write(name.as_bytes())
        .expect("Cannot write to file");
}
