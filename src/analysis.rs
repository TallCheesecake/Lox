use miette::NarratableReportHandler;

use crate::parser::{self, Tree};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Stack {
    pub scope: Vec<Scope>,
    pub flook: FLookup,
}

/*
maybe we can try and convert all the funciton and funtion call nodes
with a pointer to the  lookup table
for the ssa we really only need to be able to check it somehting exists
we dont need a bunch of data about the funciton parameters
that all has  to be stored in the lookup table
and then when we traverse the tree into ssa form
when we find a function or a call we just to into the
table, (maybe bit lookup? ) and from there we can pull the
information out parsing it strait to IR.
How to actuall yrepresent the SSA, i have litteraly no idea
 * */
#[derive(Debug)]
pub struct FLookup {
    pub table: HashMap<Rc<Tree>, Rc<Tree>>,
}
#[derive(Debug)]
pub struct Scope {
    pub scope: HashMap<String, Rc<Tree>>,
}

pub trait Visitor {
    fn visit_stmnt(&mut self, var: &Tree);
}
impl Visitor for Stack {
    fn visit_stmnt(&mut self, var: &Tree) {
        match var {
            Tree::Nil => todo!(),
            Tree::Var(op, trees) => {
                self.scope
                    .last_mut()
                    .unwrap()
                    .scope
                    .insert(String::from(op), Rc::clone(trees));
            }
            Tree::Atom(atom) => {}
            Tree::Call { callee, arguments } => {
                let call = Rc::clone(callee);
                let table = self.flook.table;
            }
            Tree::Fun {
                name,
                parameters,
                body,
            } => {
                //TODO: Put into func call look up table
                self.flook.table.insert(name, body);
                self.visit_stmnt(body.as_ref());
            }
            Tree::NonTerm(op, trees) => {
                if *op == parser::Op::Group {
                    self.scope.push(Scope::new());
                    for i in trees {
                        self.visit_stmnt(i);
                    }
                    self.scope.pop();
                }
            }
            Tree::Op(_) => {}
            Tree::ExprStatment(x) => {
                self.scope.push(Scope::new());
                for i in x {
                    self.visit_stmnt(i);
                }
                self.scope.pop();
            }
        }
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            scope: HashMap::new(),
        }
    }
}

impl Stack {
    pub fn new() -> Self {
        let scope = vec![Scope::new()];
        let flook = FLookup::new();
        Self { scope, flook }
    }
}

impl FLookup {
    pub fn new() -> Self {
        let table = HashMap::new();
        Self { table: table }
    }
}
