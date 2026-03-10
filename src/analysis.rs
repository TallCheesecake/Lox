use crate::parser::{self, Tree};
use std::{collections::HashMap, rc::Rc};
#[derive(Debug)]
pub struct Resolver {
    pub current_id: usize,
    //Var scopes
    pub vscope: Vec<Scope>,
    //Funcitons as declared in global scope
    pub fscope: Vec<FunctionInfo>,
    //Funcitons calls
    pub table: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub airity: usize,
    pub body: Rc<Tree>,
}
#[derive(Debug)]
pub struct Scope {
    pub vlookup: HashMap<String, Rc<Tree>>,
}

pub trait Visitor {
    fn visit_first(&mut self, var: &Tree);
    fn visit_second(&mut self, var: &Tree);
}

impl Visitor for Resolver {
    fn visit_first(&mut self, var: &Tree) {
        match var {
            Tree::Fun {
                name,
                parameters,
                body,
            } => {
                //This currently does not support closures :)
                let val = FunctionInfo {
                    airity: parameters.len(),
                    body: Rc::clone(body),
                };
                let out = match &**name {
                    Tree::Atom(atom) => atom,
                    _ => unreachable!(),
                };
                let name = match out {
                    parser::Atom::Ident { range, source } => {
                        let val = &(*source).as_str()[range.start..range.end];
                        String::from(val)
                    }
                    _ => unreachable!(),
                };
                self.current_id += 1;
                //add name of fun and order it was declared
                self.table.insert(name, self.current_id);
                //PUSH ONTO STACK
                self.fscope.push(val);
            }
            x => {}
        }
    }
    fn visit_second(&mut self, var: &Tree) {
        match var {
            Tree::Nil => todo!(),
            Tree::ExprStatment(trees) => {
                self.vscope.push(Scope::new());
                for i in trees {
                    self.visit_second(i);
                }
                self.vscope.pop();
            }
            Tree::Var(op, trees) => {
                self.vscope
                    .last_mut()
                    .unwrap()
                    .vlookup
                    .insert(String::from(op), Rc::clone(trees));
            }
            Tree::Call { callee, arguments } => {
                println!("THE CALL IS: call: {callee:?} args: {arguments:?}");
                let out = match &**callee {
                    Tree::Atom(atom) => atom,
                    _ => unreachable!(),
                };
                let name = match out {
                    parser::Atom::Ident { range, source } => {
                        let val = &(*source).as_str()[range.start..range.end];
                        String::from(val)
                    }
                    _ => unreachable!(),
                };

                let len = arguments.len();
                println!("found args; {:?}", arguments);

                match self.table.get(&name) {
                    Some(x) => {
                        let function = self.fscope.get(*x).unwrap();
                        if len == function.airity {
                            println!("FUNCTION LEN: {} AIR {}", len, function.airity);
                            println!("FUNCTION: {} IS FULLY QUALIFIED", name);
                            {}
                        } else {
                            panic!("func wrong num args")
                        }
                    }
                    None => {
                        println!("func: {} not in scope", name)
                    }
                }
            }
            Tree::Atom(_) => {}
            Tree::NonTerm(op, trees) => {
                if *op == parser::Op::Group {
                    self.vscope.push(Scope::new());
                    for i in trees {
                        self.visit_second(i);
                    }
                    println!("before we pop: {:?}", self.vscope);
                    self.vscope.pop();
                }
                println!("non term scope: {:?}", self.vscope);
            }
            Tree::Op(_) => {}
            _ => {}
        }
    }
}

impl FunctionInfo {
    pub fn new() -> Self {
        Self {
            airity: 0,
            body: Rc::new(Tree::Nil),
        }
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            vlookup: HashMap::new(),
        }
    }
}
impl Resolver {
    pub fn new() -> Self {
        Self {
            vscope: vec![Scope::new()],
            fscope: vec![FunctionInfo::new()],
            table: HashMap::new(),
            current_id: 0,
        }
    }
    pub fn resolve(&mut self, input: Vec<Tree>) {
        println!("MASTER INPUT : {input:?}");
        for i in input {
            self.visit_first(&i);
            self.visit_second(&i);
        }
    }
}
