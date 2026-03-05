use crate::parser::{self, *};
use std::rc::Rc;
//

#[derive(Debug)]
pub struct Scope {
    level: u64,
    group: Rc<Tree>,
}

//And other parts of the resolver
#[derive(Debug)]
pub struct STree {
    elem: parser::Tree,
}

impl STree {
    pub fn new(mut input: Parser) -> Result<(), miette::Report> {
        for i in input.parse_program()? {}
        Ok(())
    }
}
