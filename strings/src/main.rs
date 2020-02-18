mod pig_latin;

fn main() {
    let test = String::from("first apple");

    println!("pig latin: {}", pig_latin::convert_to_pig_latin(&test));
}
