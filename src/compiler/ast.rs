use super::clexor;

#[derive(Clone)]
pub enum NodeType {
    FunctionDeclaration,
    Value,
    Return,
    Statement,
    Operation
}

pub enum Operations {
    LogicalNegation
}

/*Will turn into trait*/
pub struct ASTNode {
    pub function: NodeType,
    pub value: Option<String>,
    nodes: Option<Vec<ASTNode>>
}

impl ASTNode {
    pub fn new(nodetype: NodeType, value: Option<String>) -> ASTNode {
        let new_node = ASTNode {
            function: nodetype,
            value,
            nodes: None
        };

        new_node
    }

    pub fn SetValue(&mut self, value: String) {
        self.value = Some(value);
    }

    pub fn AddNode(&mut self, node: ASTNode) {
        match self.nodes.as_ref() {
            Some(nodes) => {

            },
            None => {
                self.nodes = Some(Vec::new());

            }
        }
    }
}
/*

pub fn AST_to_assembly(function: CFunction) -> String {

}
*/
