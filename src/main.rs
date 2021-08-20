
fn reverse_string(phrase: &str) -> String {
   phrase.chars().rev().collect()
}



fn main() {
    let reversed = reverse_string("Hello World");
    println!("{}", reversed);
}



#[cfg(test)]
pub mod tests {
    #[test]
    pub fn test_reverse_string() {
        assert_eq!(reverse_string("world"), "dlrow");
    }
}
