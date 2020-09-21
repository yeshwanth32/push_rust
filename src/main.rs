#![allow(warnings)]

use pest;
use pest_consume::{match_nodes, Error, Parser};

type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Parser)]
#[grammar = "push_grammar.pest"]
pub struct pharos_parser;

fn main() {
    println!("Hello, world!");
}


#[pest_consume::parser]
impl pharos_parser {
    fn string(_input: Node) -> Result<(), Error<Rule>> {
        let s = _input.as_str();
        println!("{}", s);
        Ok(())
    }
    fn number(_input: Node) -> Result<(), Error<Rule>> {
        let num = _input
            .as_str()
            .parse::<f64>()
            .map_err(|e| _input.error(e))?;
        //println!("inside number {:?}", num);
        println!("{}", num);
        Ok(())
    }
    fn boolean(_input: Node) -> Result<(), Error<Rule>> {
        let b = _input
            .as_str()
            .parse::<bool>()
            .map_err(|e| _input.error(e))?;
        println!("{}", b);
        Ok(())
    }
    
}

