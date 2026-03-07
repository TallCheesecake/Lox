use crate::parser::{self, Tree, *};
use miette::highlighters::Highlighter;
use std::{cell::RefCell, collections::HashMap, fs::exists, i32, rc::Rc};
#[derive(Debug)]
pub struct Scope {
    level: u32,
    stack: Vec<RefCell<HashMap<u32, String>>>,
}
pub trait Visitor<T> {
    fn visit_nonterm(&mut self, var: &[Tree]) -> Option<()>;
    fn visit_var(&mut self, var: &[Tree]);
}

impl Visitor<String> for Scope {
    fn visit_nonterm(&mut self, var: &[Tree]) -> Option<()> {
        for i in var {
            match *i {
                Tree::Nil => None,
                Tree::NonTerm(_, ref trees) => self.visit_nonterm((trees).as_slice()),
                Tree::Var(_, ref trees) => Some(self.visit_var(trees.as_slice())),
                Tree::Atom(_) => None,
                Tree::ExprStatment(ref tree) => self.visit_nonterm((tree).as_slice()),
                _ => unimplemented!(),
            };
        }
        None
    }

    fn visit_var(&mut self, var: &[Tree]) {
        for i in var {
            match *i {
                Tree::Var(ref z, ref trees) => {
                    for i in self.stack.as_slice() {
                        i.borrow_mut().insert(self.level, String::from(z));
                    }
                    self.visit_nonterm(trees.as_slice());
                }
                _ => unimplemented!(),
            }
        }
    }
}
//NEW THING: Visitor pattern
static SCOPE: i32 = 0;
impl Scope {
    fn new() -> Self {
        Self {
            level: 0,
            stack: Vec::new(),
        }
    }

    fn check(&mut self, val: u32) -> bool {
        for i in self.stack.as_slice() {
            if i.borrow_mut().contains_key(&val) {
                return true;
            } else {
                return false;
            }
        }
        false
    }
}
