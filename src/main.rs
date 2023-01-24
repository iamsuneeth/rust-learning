use std::env;

mod box_sum;
mod count_words;
mod display_trait;
mod find_name;
mod guess_random;
mod guess_random_handle_error;
mod location;
mod shape;

fn main() {
    if env::args().len() < 2 {
        println!("Please provide the learning name to run");
        return;
    }

    let learning_name = env::args().nth(1).unwrap();
    match learning_name.as_str() {
        "guess_number" => guess_random::guess_number(),
        "find_name" => find_name::find_name(),
        "shape" => shape::test_shape(),
        "box_sum" => box_sum::test_box_sum(),
        "display_trait" => display_trait::test_display(),
        "print_location" => location::print_location(),
        "guess_random_handle_error" => guess_random_handle_error::guess_number(),
        "count_words" => count_words::print_word_count(),
        _ => panic!("No learning exists"),
    }
}
