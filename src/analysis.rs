use crate::parser::{self, Tree};
use core::panic;
use miette::{Context, Diagnostic, Result, SourceSpan};
use std::{collections::HashMap, rc::Rc, sync::Arc};
#[derive(Debug)]
pub struct Resolver {
    pub ast: parser::Parser,
    pub current_id: usize,
    //Var scopes
    pub vscope: Vec<Scope>,
    //Funcitons as declared in global scope
    pub fscope: Vec<FunctionInfo>,
    //Funcitons calls
    pub table: HashMap<String, usize>,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(help("this is a semantics/resolution error"))]
pub struct ResolverError {
    #[source_code]
    source: Arc<String>,
    #[label("main issue")]
    primary_span: SourceSpan,
}
impl std::error::Error for ResolverError {}
impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
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
    fn visit_first(&mut self, var: &Tree) -> Result<(), miette::Report>;
    fn visit_second(&mut self, var: &Tree) -> Result<(), miette::Report>;
}
impl Visitor for Resolver {
    fn visit_first(&mut self, var: &Tree) -> Result<(), miette::Report> {
        match var {
            Tree::Var(op, trees) => {
                self.vscope
                    .last_mut()
                    .unwrap()
                    .vlookup
                    .insert(String::from(op), Rc::clone(trees));
            }
            Tree::Fun {
                name,
                parameters,
                body,
            } => {
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
                    x => return self.error("you cant make this a variable name: ", Some(x)),
                };
                self.table.insert(name, self.current_id);
                self.fscope.push(val);
            }
            _ => {}
        }
        self.visit_second(var)?;
        Ok(())
    }

    fn visit_second(&mut self, var: &Tree) -> Result<(), miette::Report> {
        match var {
            Tree::Nil => todo!(),
            Tree::ExprStatment(trees) => {
                self.vscope.push(Scope::new());
                for i in trees {
                    self.visit_first(i)?;
                }
                self.vscope.pop();
            }
            Tree::Var(_, _) => {}
            Tree::Call { callee, arguments } => {
                let out = match &**callee {
                    Tree::Atom(atom) => atom,
                    _ => unreachable!(),
                };
                let name = match out {
                    parser::Atom::Ident { range, source } => {
                        let val = &(*source).as_str()[range.start..range.end];
                        String::from(val)
                    }
                    x => {
                        return self
                            .error("function must be declared before they are used: ", Some(x));
                    }
                };

                let len = arguments.len();

                match self.table.get(&name) {
                    Some(x) => {
                        let function = self.fscope.get(*x).unwrap();
                        self.current_id += 1;
                        if len == function.airity {
                            println!("FUNCTION: {} IS FULLY QUALIFIED", name);
                            {}
                        } else {
                            println!("FUNCTION: {} IS NOT FULLY QUALIFIED", name);
                            panic!("func wrong num args")
                        }
                    }

                    None => {
                        let val = format!("the varible:  {} is not in scope", name);
                        return self.error(&val, None);
                    }
                }
            }
            Tree::Atom(x) => {
                let output = match self.resolve_atm(x) {
                    Some(x) => x,
                    None => return Ok(()),
                };
                self.vscope.reverse();
                for i in &self.vscope {
                    if i.vlookup.contains_key(&output) {
                        {
                            println!("variable with name of: {output}, maps to : {i:?} ");
                            return Ok(());
                        }
                    }
                }
                return self.error("this varible must be declared before use ", Some(x));
            }
            Tree::NonTerm(op, trees) => {
                if *op == parser::Op::Group {
                    self.vscope.push(Scope::new());
                }
                for i in trees {
                    self.visit_first(i)?;
                }
                self.vscope.pop();
            }
            Tree::Op(_) => {}
            _ => {}
        }
        Ok(())
    }
}

impl FunctionInfo {}

impl Scope {
    pub fn new() -> Self {
        Self {
            vlookup: HashMap::new(),
        }
    }
}
impl Resolver {
    pub fn new(ast: parser::Parser) -> Self {
        Self {
            ast,
            vscope: vec![Scope::new()],
            fscope: vec![],
            table: HashMap::new(),
            current_id: 0,
        }
    }
    pub fn resolve_atm(&mut self, input: &parser::Atom) -> Option<String> {
        let out = match input {
            parser::Atom::Ident { range, source } => {
                let val = &(*source).as_str()[range.start..range.end];
                Some(String::from(val))
            }
            parser::Atom::String { range, source } => None,
            parser::Atom::Number(x) => None,
            parser::Atom::Bool(_) => None,
            _ => unreachable!(),
        };
        out
    }
    pub fn resolve(&mut self) -> Result<(), miette::Report> {
        let ast = self.ast.parse_program()?;
        for i in ast {
            self.visit_first(&i)?;
        }
        Ok(())
    }
    fn error(&mut self, message: &str, found: Option<&parser::Atom>) -> Result<(), miette::Report> {
        let err = match found {
            Some(x) => {
                format!("{message}: `{}`", x)
            }
            None => {
                format!("{message}")
            }
        };
        return Err(ResolverError {
            source: Arc::clone(&self.ast.input),
            primary_span: parser::token_to_span(&self.ast.current()),
        })
        .wrap_err(err);
    }
}
