#![allow(unused_variables, dead_code)]

mod bigints;
mod datatype;
mod duplicate_ty_name;
mod export;
mod functions;
mod macro_decls;
mod reserved_keywords;
mod selection;
pub mod ts;
mod ts_rs;
mod ty_override;

#[test]
fn test_compile_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/macro/compile_error.rs");
}
