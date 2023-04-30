#[cfg(test)]
mod tests {
    #[test]
    fn contains_test() {
        let input = "world";
        assert_ne!(crate::utils::contains('x', input), true);
    }

    #[test]
    fn loads_dataset() {
        let expected = vec!["BILL", "GATE", "BEST", "GENT", "EVER"];

        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // construcing a relative path from PathBuff
        path.push("words/test.txt");
        
        let result = crate::utils::import_dataset("test", 0);
        assert_eq!(expected, result);
    }
}
