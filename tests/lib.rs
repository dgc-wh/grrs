
#[test]
fn test_find_matches() {
        let mut result = Vec::new();
        grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).expect("failed to find");
        assert_eq!(result, b"lorem ipsum\n");
}
