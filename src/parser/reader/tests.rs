use crate::parser::Reader;

#[test]
fn test_read_int() {
    let mut reader = Reader::new("12");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_float() {
    let mut reader = Reader::new("12.2");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_rational() {
    let mut reader = Reader::new("12/2");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_char() {
    let mut reader = Reader::new("\'a\'");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_string() {
    let mut reader = Reader::new("\"Hello, world!\"");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_bool() {
    let mut reader = Reader::new("true");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_symbol() {
    let mut reader = Reader::new("foo");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_add_list() {
    let mut reader = Reader::new("(+ 1 2)");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_nested_list() {
    let mut reader = Reader::new("(+ (% 5 3) 1 2)");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}
