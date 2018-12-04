use super::clexor;

#[derive(Clone)]
pub enum NodeType {
    FunctionDeclaration,
    Return
}

pub trait ASTNode {
    fn num_branching(&self) -> u32;
    fn to_string(&self) -> String;
    fn get_node_type(&self) -> NodeType;
    fn get_value(&self) -> String;
}

pub struct ReturnNode {
    pub type_value: String,
    pub constant: bool,
    pub value: String,
    pub node_type: NodeType
}

pub struct FunctionDeclarationNode {
    pub name: String,
    pub value_type: clexor::Keyword,
    pub node_type: NodeType
}

impl ASTNode for ReturnNode {
    fn num_branching(&self) -> u32 {
        0
    }

    fn to_string(&self) -> String {
        format!("RETURN {}<{}>", self.type_value, self.value)
    }

    fn get_node_type(&self) -> NodeType {
        self.node_type.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl ReturnNode {
    fn new(type_value: String, value: String, is_constant: bool) -> ReturnNode {
        let new_node = ReturnNode {
            type_value,
            constant: is_constant,
            value,
            node_type: NodeType::Return
        };

        new_node
    }
}

impl ASTNode for FunctionDeclarationNode {
    fn num_branching(&self) -> u32{
        0
    }

    fn to_string(&self) -> String {
        format!("FUN {} {}:\n\tparams: ()\n", self.value_type.to_string() , self.name)
    }

    fn get_node_type(&self) -> NodeType {
        self.node_type.clone()
    }

    fn get_value(&self) -> String {
        self.name.clone()
    }
}

impl FunctionDeclarationNode {
    fn new(name: String, value_type: clexor::Keyword) -> FunctionDeclarationNode {
        let new_node = FunctionDeclarationNode {
            name,
            value_type,
            node_type: NodeType::FunctionDeclaration
        };

        new_node
    }
}

pub fn lex_to_ast(function: clexor::CFunction) -> Vec<Box<dyn ASTNode>> {
    let mut ast: Vec<Box<dyn ASTNode>> = Vec::new();
    let function_node = Box::new(FunctionDeclarationNode::new(function.name, function.keyword));
    let return_node = Box::new(ReturnNode::new(clexor::get_keyword_str(function.return_value.keyword), function.return_value.value, function.return_value.is_constant));

    ast.push(function_node);
    ast.push(return_node);

    ast
}