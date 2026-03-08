//NOTE: I wanted to do closure but this means that we
//have to be able to look back in the tree/stack
//this means some play on a ll with either Rc and refcell
//or I cook it up with unsafe.
//This sounds like it sucks and I will not be doing it NOW
//it may in the future
use crate::parser::{self, Tree};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Stack {
    pub scope: Vec<Scope>,
}
#[derive(Debug)]
pub struct Scope {
    pub scope: HashMap<String, Rc<Tree>>,
}

pub trait Visitor {
    fn visit_stmnt(&mut self, var: &Tree);
    fn visit_expr(&mut self, var: &Tree);
    fn visit_var(&mut self, var: &Tree);
}

//Eachtime we see a group we have to add a new scope
//eachtime we see a var we have to add it to the
//current scope
//each time we leave a scope we pop it from the
//stack
impl Visitor for Stack {
    fn visit_stmnt(&mut self, var: &Tree) {
        match var {
            Tree::Nil => todo!(),
            Tree::Var(op, trees) => {
                println!("trees: {:?}", trees);
                println!("self: {:?}", self.scope);
                self.scope
                    .last_mut()
                    .unwrap()
                    .scope
                    .insert(String::from(op), Rc::clone(trees));
            }

            Tree::Atom(atom) => {}
            Tree::NonTerm(op, trees) => {
                if *op == parser::Op::Group {
                    self.scope.push(Scope::new());
                    for i in trees {
                        self.visit_stmnt(i);
                    }
                    println!("old scope  {:?}", self.scope);
                    self.scope.pop();
                }
            }
            Tree::Op(op) => todo!(),
            _ => todo!(),
        }
    }
    fn visit_expr(&mut self, var: &Tree) {
        todo!()
    }
    fn visit_var(&mut self, var: &Tree) {
        todo!()
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            scope: HashMap::new(),
        }
    }
    pub fn add_to_scope(&mut self, k: &str, v: Rc<Tree>) {
        self.scope.insert(String::from(k), v);
    }
    pub fn resolve(&mut self, k: &str) -> bool {
        self.scope.contains_key(k)
    }
}

impl Stack {
    pub fn new() -> Self {
        let mut temp = vec![Scope::new()];
        Self { scope: temp }
    }
}
