use super::clexor;

pub struct FunctionNode {

}

pub struct ReturnNode {
    return_value: clexor::Value
}

pub trait ASTNode {
    fn number_own_branching_nodes(&self) -> u32;
    fn string_format(&self) -> String;
}

impl ASTNode for ReturnNode {
    fn number_own_branching_nodes(&self) ->u32 {
        0
    }

    fn string_format(&self) -> String{
        let result = format!("RETURN {}<{}>", &self.return_value.keyword, &self.return_value.value);
        result
    }
}

fn lex_to_ast (function: &clexor::CFunction){

}
