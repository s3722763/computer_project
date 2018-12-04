use pest::Parser;
use pest::iterators::Pairs;

use std::fs;
use std::fmt;

#[derive(Parser)]
#[grammar="../c.pest"]
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

#[derive(Clone)]
pub struct CFunction {
    pub keyword: Keyword,
    pub name: String,
    pub return_value: Value
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
            return_value: Value::new()
        };

        result
    }
}

fn get_keyword(keyword: &str) -> Result<Keyword, KeywordError> {
    let result: Result<Keyword, KeywordError>;

    if keyword == "int" {
        result = Ok(Keyword::Int);
    } else if keyword == "void" {
        result = Ok(Keyword::Void);
    } else {
        let reason = format!("{} {} {}", "Keyword", keyword,"does not exist");
        let error = KeywordError::new(reason);
        result = Err(error);
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
                function.return_value.value = value.to_string();
                function.return_value.is_constant = true;
                function.return_value.keyword = Keyword::Int;
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
                //println!("Name: {}\tReturn: {}", function.name, function.return_value.value);
                list_of_functions.push(function);
            }
            _ => {}
        }
    }

    list_of_functions
}
