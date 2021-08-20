
fn reverse_string(phrase: &str) -> String {
   phrase.chars().rev().collect()
}

fn bool_to_word(value: bool) -> &'static str {
    match value {
        true => {
            "Yes"
        }
        false => {
            "No"
        }
    }
}



fn main() {
    let reversed = reverse_string("Hello World");
    println!("{}", reversed);

    let word_from_bool = bool_to_word(true);
    println!("{}", word_from_bool)
}



#[cfg(test)]
pub mod tests {

    #[test]
    pub fn test_reverse_string() {
        assert_eq!(reverse_string("world"), "dlrow");
    }

    #[test]
    fn test_bool_to_word() {
        assert_eq!(bool_to_word(true), "Yes");
        assert_eq!(bool_to_word(false), "No");
    }
}
