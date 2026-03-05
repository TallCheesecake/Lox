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
//Scope is to be stack based
//Push a new scope on when you see a group op
//
//you assign a SSA ID with each iden
//then you make a Hmap of <SSAID, var value>
// I guess you can store a pointer into that hmap on the stack and
// if you what to know if a var is in scope: you start at the
// top of the stack and you access the hmap pointer to see
// if you have the SSAID you want access to 
// so like if you are calling x_1 or x_2
// if the hashmap returns a value then you are good
// when we leave the stack we pop it
// this means that we will always only be able to see
// parent scopes 
impl STree {
    pub fn new(mut input: Vec<Tree>) -> Result<STree, miette::Report> {
        loop {
            match input.
        }
    }
}
