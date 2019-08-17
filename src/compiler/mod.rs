pub mod clexor;
pub mod ast;

pub fn compile(file_path: &str) {
    let functions = clexor::parse(file_path);

    //let ast_tree = ast::lex_to_ast(functions.get(0).unwrap().clone());
    for function in functions {
        let asm = ast::function_to_assembly(function);
        println!("{}", asm);
    }
    //let assembly_code = compile_ast(ast_tree);

    //assembly_code
}
