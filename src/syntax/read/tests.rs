// #[test]
// fn test_read_int() {
//     let tokens = TokenStream::new("12").expect("Failed to lex tokens");
//     let mut reader = Reader::new(tokens);
//     let sexpr = reader.sexpr().expect("Failed to read sexpr");
//     insta::assert_debug_snapshot!(sexpr);
// }

// #[test]
// fn test_read_rational() {
//     let tokens = TokenStream::new("12/2").expect("Failed to lex tokens");
//     let mut reader = Reader::new(tokens);
//     let sexpr = reader.sexpr().expect("Failed to read sexpr");
//     insta::assert_debug_snapshot!(sexpr);
// }

// #[test]
// fn test_read_real() {
//     let tokens = TokenStream::new("12.0").expect("Failed to lex tokens");
//     let mut reader = Reader::new(tokens);
//     let sexpr = reader.sexpr().expect("Failed to read sexpr");
//     insta::assert_debug_snapshot!(sexpr);
// }

// #[test]
// fn test_read_char() {
//     let tokens = TokenStream::new("\'a\'").expect("Failed to lex tokens");
//     let mut reader = Reader::new(tokens);
//     let sexpr = reader.sexpr().expect("Failed to read sexpr");
//     insta::assert_debug_snapshot!(sexpr);
// }

// #[test]
// fn test_read_string() {
//     let tokens = TokenStream::new("\"Hello, World!\"").expect("Failed to lex tokens");
//     let mut reader = Reader::new(tokens);
//     let sexpr = reader.sexpr().expect("Failed to read sexpr");
//     insta::assert_debug_snapshot!(sexpr);
// }

// #[test]
// fn test_read_add_list() {
//     let tokens = TokenStream::new("(+ 1 2)").expect("Failed to lex tokens");
//     let mut reader = Reader::new(tokens);
//     let sexpr = reader.sexpr().expect("Failed to read sexpr");
//     insta::assert_debug_snapshot!(sexpr);
// }

// #[test]
// fn test_read_nested_list() {
//     let tokens = TokenStream::new("(+ (% 5 3) 1 2)").expect("Failed to lex tokens");
//     let mut reader = Reader::new(tokens);
//     let sexpr = reader.sexpr().expect("Failed to read sexpr");
//     insta::assert_debug_snapshot!(sexpr);
// }
