use pest::Parser;
use pest::iterators::Pairs;

use std::fs;
use std::fmt;

#[derive(Parser)]
#[grammar="../c.pest"]
pub struct CParser;

pub struct Value {
    pub keyword: Keyword,
    pub value: String,
    pub is_constant: bool
}

#[derive(PartialEq)]
pub enum Keyword{
    Int,
    None
}

pub struct CFunction {
    pub keyword: Keyword,
    pub name: String,
    pub return_value: String,
}

pub struct KeywordError {
    reason: String,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string_rep = "None";

        if *self == Keyword::Int {
            string_rep = "int";
        }

        write!(f, "{}", string_rep)
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
            return_value: String::from("")
        };

        result
    }
}

fn get_keyword(keyword: &str) -> Result<Keyword, KeywordError>{
    let result: Result<Keyword, KeywordError>;

    if keyword == "int" {
        result = Ok(Keyword::Int);
    } else {
        let reason = format!("{} {} {}", "Keyword", keyword,"does not exist");
        let error = KeywordError::new(reason);
        result = Err(error);
    }

    result
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
                    Ok(key) => {
                        if function.keyword == Keyword::None {
                            function.keyword = key;
                        } else {
                            println!("Function {} already as a keyword. Cannot define multiple.", function.name);
                            error_count = error_count + 1;
                        }
                    }
                    Err(err) => {
                        println!("Function {}: {}", function.name, err.reason);
                        error_count = error_count + 1;
                    }
                }
            }
            Rule::function_name => {
                let value = pair.as_str();
                if function.name == "" {
                    function.name = value.to_string();
                } else {
                    println!("{}", "Invalid function name");
                    error_count = error_count + 1;
                }
            }
            Rule::return_statement => {
                let value = pair.into_inner().as_str();
                function.return_value = value.to_string();
            }
            _ =>{
                println!("{}", "Invalid rule");
            }
        }
    }

    function
}
pub fn parse(file_path: &str) -> Vec<CFunction>{
    let c_file = fs::read_to_string(file_path).expect("Could not read file");
    let parse_file = CParser::parse(Rule::file, &c_file).expect("Could not parse").next().unwrap();
    let mut list_of_functions: Vec<CFunction> = Vec::new();

    for line in parse_file.into_inner() {
        match line.as_rule() {
            Rule::function => {
                let function = parse_function(line.into_inner());
                println!("Name: {}\tReturn: {}", function.name, function.return_value);
                list_of_functions.push(function);
            }
            _ => {}
        }
    }

    list_of_functions
}
