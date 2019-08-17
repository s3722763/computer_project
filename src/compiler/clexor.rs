use super::ast::{ASTNode, NodeType};

use pest::Parser;
use pest::iterators::Pairs;

use std::fs;
use std::fmt;

#[derive(Parser)]
#[grammar="../lexor/c.pest"]
pub struct CParser;

#[derive(Clone)]
pub struct Value {
    pub keyword: Keyword,
    pub value: String,
    pub is_constant: bool
}

#[derive(PartialEq, Clone)]
pub enum Keyword{
    Int,
    Void,
    None
}

pub struct CFunction {
    pub keyword: Keyword,
    pub name: String,
    pub statements: Option<Vec<ASTNode>>
}

pub struct KeywordError {
    reason: String,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_rep = "None";

        if *self == Keyword::Int {
            string_rep = "int";
        } else if *self == Keyword::Void {
            string_rep = "void";
        }

        write!(f, "{}", string_rep)
    }
}

impl Value {
    pub fn new() -> Value {
        let result = Value {
            keyword: Keyword::None,
            value: String::from(""),
            is_constant: false
        };

        result
    }
}

impl KeywordError {
    pub fn new(reason_s: String) -> KeywordError{
        let error = KeywordError {reason: reason_s};
        error
    }
}

impl CFunction {
    pub fn new() -> CFunction {
        let result = CFunction {
            keyword: Keyword::None,
            name: String::from(""),
            statements: None
        };

        result
    }

    pub fn AddStatement(&mut self, node: ASTNode) {
        match &mut self.statements {
            Some(nodes) => {
                nodes.push(node);
            },
            None => {
                let mut new_tree: Vec<ASTNode> = Vec::new();
                new_tree.push(node);
                self.statements = Some(new_tree);
            }
            _ => {}
        }
    }
}

fn get_keyword(keyword: &str) -> Option<Keyword> {
    let mut result = None;

    if keyword == "int" {
        result = Some(Keyword::Int);
    } else if keyword == "void" {
        result = Some(Keyword::Void);
    }

    result
}

pub fn get_keyword_str(keyword: Keyword) -> String {
    let result: &str;

    match keyword {
        Keyword::Int => result = "int",
        Keyword::Void => result = "void",
        _ => result = "err"
    };

    result.to_string()
}

fn parse_function(function_rules: Pairs<Rule>) -> CFunction{
    //println!("{}", function_rules);
    let mut function = CFunction::new();
    let mut error_count = 0;

    for pair in function_rules {
        match pair.as_rule() {
            Rule::keyword => {
                let value = pair.as_str();
                let keyword = get_keyword(value);
                match keyword {
                    Some(key) => {
                        if function.keyword == Keyword::None {
                            function.keyword = key;
                        } else {
                            println!("Function {} already as a keyword. Cannot define multiple.", function.name);
                            error_count = error_count + 1;
                        }
                    },
                    _ => {}
                }
            }
            Rule::function_name => {
                let value = pair.as_str();
                if function.name.as_str() == "" {
                    function.name = value.to_string();
                } else {
                    println!("{}", "Function name already set");
                    error_count = error_count + 1;
                }
            }
            Rule::statement => {
                let statement_node = parse_statement(pair.into_inner());
                function.AddStatement(statement_node);
                //parse_statement();
            }
            _ =>{
                println!("{}", "Invalid rule");
            }
        }
    }

    function
}

fn parse_statement(pairs: Pairs<Rule>) -> ASTNode {
    let mut statement_node = ASTNode::new(NodeType::Statement, None);

    for pair in pairs {
        match pair.as_rule() {
            Rule::operation => {},
            Rule::return_statement => {
                let return_value = String::from(pair.as_str());
                let node = ASTNode::new(NodeType::Return, Some(return_value));

                statement_node.AddNode(node);

            },
            _ => {}
        }
    }

    statement_node
}

fn parse_operation(pairs: Pairs<Rule>) -> ASTNode {
    let mut node = ASTNode::new(NodeType::Operation, None);

    node
}

pub fn parse(file_path: &str) -> Vec<CFunction>{
    let c_file = fs::read_to_string(file_path).expect("Could not read file");
    let parse_file = CParser::parse(Rule::file, &c_file).expect("Could not parse").next().unwrap();
    let mut list_of_functions: Vec<CFunction> = Vec::new();

    for line in parse_file.into_inner() {
        match line.as_rule() {
            Rule::function => {
                let function = parse_function(line.into_inner());
                //println!("Name: {}\tReturn: {}", function.name, function.return_value.value);
                list_of_functions.push(function);
            },
            _ => {}
        }
    }

    list_of_functions
}
