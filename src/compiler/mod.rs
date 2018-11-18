pub mod clexor;
pub mod ast;

pub fn compile(file_path: &str) {
    let functions = clexor::parse(file_path);
}
