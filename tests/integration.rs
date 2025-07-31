pub fn format_filename(s: &str) -> String {
    // Windows sucks
    // <>:"/\|?*
    s.replace("<", "")
        .replace(">", "")
        .replace(":", "")
        .replace("/", "")
        .replace("\\", "")
        .replace("|", "")
        .replace("?", "")
        .replace("*", "")
        .replace("\n", "")
}

mod codegen;

mod parser;

mod semantics;
