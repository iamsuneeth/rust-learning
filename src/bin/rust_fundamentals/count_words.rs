use std::{collections::HashMap, env, fs, io};

pub fn print_word_count() {
    if env::args().len() < 3 {
        println!("Plase provide the file path to count words from");
        return;
    }
    let file_path = env::args().nth(2);
    let file_path = match file_path {
        Some(name) => name,
        None => {
            println!("Plase provide file name to count words from");
            return;
        }
    };
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => {
                print!("File not found");
                return;
            }
            io::ErrorKind::PermissionDenied => {
                print!("Permission denied");
                return;
            }
            _ => panic!("Some other error"),
        },
    };
    let words = content.split_whitespace();
    let mut wordd_count_map = HashMap::new();
    for word in words {
        wordd_count_map
            .entry(word)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let popular = wordd_count_map.iter().max_by(|a, b| a.1.cmp(&b.1));
    match popular {
        Some(popular) => println!("most popular is {} with count {}", popular.0, popular.1),
        None => println!("No word is popular"),
    }
}
