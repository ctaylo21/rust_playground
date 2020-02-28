use std::io;

mod pig_latin;

fn main() {
    println!("Please enter a phrase to convert to pig latin!");

    let mut phrase = String::new();

    io::stdin()
        .read_line(&mut phrase)
        .expect("Failed to read line");

    println!("pig latin: {}", pig_latin::convert_to_pig_latin(&phrase));
}
