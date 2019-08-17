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
        match &mut self.nodes {
            Some(nodes) => {
                nodes.push(node);
            },
            None => {
                let mut new_tree: Vec<ASTNode> = Vec::new();
                new_tree.push(node);
                self.nodes = Some(new_tree);
            }
            _ => {}
        }
    }
}

pub fn function_to_assembly(function: clexor::CFunction) -> String {
    let mut asm = String::new();

    if function.name.eq_ignore_ascii_case("main") {
        asm.push_str(".glob _main");
        asm.push_str("_main:");
    }

    for statement in function.statements {
        statement_to_assembly(statement);
    }

    asm
}

fn statement_to_assembly(statement: Vec<ASTNode>) -> String {
    let mut asm_temp = String::new();

    for part in statement {
        match part.nodes {
            Some(p) => {
                statement_to_assembly(p);
            },
            None => {
                /*If it gets here, means that the statement does not contain any more parts*/
                match part.function {
                    NodeType::Return => {
                        match part.value {
                            Some(p) => {
                                asm_temp.push_str(format!("movl ${}, %eax\nret\n", p).as_str());
                            },
                            None => {
                                asm_temp.push_str("ret\n");
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    return asm_temp;
}