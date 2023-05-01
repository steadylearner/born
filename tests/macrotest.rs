use macrotest::expand;

// $cargo test macros
#[test]
pub fn macros() {
    expand("tests/expand/*.rs");
}
