/// Returns the project's tagline.
pub fn get_tagline() -> String {
    "Over-Engineered For Survival".to_string()
}

/// Prints the project's tagline.
pub fn print_tagline() {
    let tagline = get_tagline();
    println!("{}", tagline);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tagline() {
        let expected = "Over-Engineered For Survival";
        let actual = get_tagline();
        assert_eq!(expected, actual);
    }
}
