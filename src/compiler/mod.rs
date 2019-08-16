pub mod clexor;
pub mod ast;

pub fn compile(file_path: &str) {
    let functions = clexor::parse(file_path);

    //let ast_tree = ast::lex_to_ast(functions.get(0).unwrap().clone());
    //let assembly_code = compile_ast(ast_tree);

    //assembly_code
}

/*
fn compile_ast(function_tree: Vec<Box<ast::ASTNode>>) -> String{
    let mut code = String::new();

    for node in function_tree {
        match node.get_node_type() {
            ast::NodeType::FunctionDeclaration => {
                code += format!("{}:\n", node.get_value()).as_str();
            },

            ast::NodeType::Return => {
                code += format!("mov\t%eax, ${}\nret\n", node.get_value()).as_str();
            }
        };
    }

    code
}
*/
