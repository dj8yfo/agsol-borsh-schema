use crate::{generate_layouts, generate_output};

#[test]
fn generate_output_from_test_directory() {
    // let layouts = generate_layouts("src/test").unwrap();
    let layouts = Vec::new();
    generate_output(&layouts, "test-rs-output-ts-input").unwrap();
}
