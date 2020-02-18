pub fn convert_to_pig_latin(text: &str) -> String {
    let mut pig_latin_text = String::from("");

    for word in text.split_whitespace() {
        match word.chars().next().unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                pig_latin_text += &format!(" {}-hay", word);
            }
            _ => {
                pig_latin_text += &format!(" {}-{}ay", &word[1..], &word[0..1]);
            }
        }
    }

    pig_latin_text.trim().to_string()
}
