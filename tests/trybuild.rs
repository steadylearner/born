use trybuild::TestCases;

// $cargo test fail
#[test]
fn fail() {
    let t = TestCases::new();
    t.compile_fail("tests/fail/*.rs");
}