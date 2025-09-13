use std::{fs, path::Path};

use insta::Settings;
use nolana::Parser;

fn with_settings(f: impl FnOnce()) {
    let mut settings = Settings::clone_current();
    settings.set_omit_expression(true);
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(f)
}

fn read_and_parse(path: &Path) -> String {
    let source = fs::read_to_string(path).unwrap();
    let result = Parser::new(&source).parse();
    format!("{result:#?}")
}

#[test]
fn test_parser() {
    with_settings(|| {
        insta::glob!("parser/*.nolana", |path| {
            insta::assert_snapshot!(read_and_parse(path));
        });
    });
}
