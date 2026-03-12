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
    //TODO: maybe add this
    //Funcitons as declared in scope
    pub fscope: Vec<FunctionInfo>,
    //Funcitons calls
    pub func_table: HashMap<String, usize>,
    //Funcitons as declared in scope
    pub cscope: Vec<ClassInfo>,
    //Class calls
    pub class_table: HashMap<String, usize>,
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

//TODO: Cont working on classes
#[derive(Debug)]
pub struct ClassInfo {
    pub body: Rc<Tree>,
    pub fields: Rc<Tree>,
    pub methods: Rc<Tree>,
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub body: Rc<Tree>,
    pub airity: usize,
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
            Tree::Var(op, trees) => self.store_scope(op, trees)?,
            Tree::NonTerm(op, trees) => if *op == parser::Op::Class {},
            Tree::Fun {
                name,
                parameters,
                body,
            } => self.store_funcitons(name, parameters, body)?,
            _ => {}
        }
        self.visit_second(var)?;
        Ok(())
    }

    fn visit_second(&mut self, var: &Tree) -> Result<(), miette::Report> {
        match var {
            Tree::Nil => todo!(),
            Tree::ExprStatment(trees) => {
                self.resolve_expr_stmnt(trees)?;
            }
            Tree::Var(op, val) => {
                self.vscope.reverse();
                println!("WE FIND: {}", op);
                for i in &self.vscope {
                    if i.vlookup.contains_key(op) {
                        {
                            println!("variable with name of: {op}, maps to : {i:?} ");
                            return Ok(());
                        }
                    }
                }
            }
            Tree::Call { callee, arguments } => {
                self.resolve_fun(callee, arguments)?;
            }
            Tree::Atom(x) => {
                self.resolve_atom(x)?;
            }
            Tree::NonTerm(op, trees) => {
                self.resolve_non_term(op, trees)?;
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
            func_table: HashMap::new(),
            current_id: 0,
            cscope: vec![],
            class_table: HashMap::new(),
        }
    }
    pub fn store_funcitons(
        &mut self,
        name: &Rc<Tree>,
        parameters: &Vec<Rc<Tree>>,
        body: &Rc<Tree>,
    ) -> Result<(), miette::Report> {
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
        self.func_table.insert(name, self.current_id);
        self.fscope.push(val);
        Ok(())
    }
    pub fn store_scope(&mut self, input: &String, trees: &Rc<Tree>) -> Result<(), miette::Report> {
        self.vscope
            .last_mut()
            .expect("invariant borked: you cant have 0 scopes")
            .vlookup
            .insert(String::from(input), Rc::clone(trees));
        Ok(())
    }
    pub fn resolve_expr_stmnt(&mut self, trees: &Vec<Tree>) -> Result<(), miette::Report> {
        self.vscope.push(Scope::new());
        for i in trees {
            self.visit_first(i)?;
        }
        self.vscope.pop();
        Ok(())
    }
    pub fn resolve_atom(&mut self, x: &parser::Atom) -> Result<(), miette::Report> {
        let output = match self.ext_value(x) {
            Some(x) => x,
            None => {
                // println!("the AST: ");
                return Ok(());
            }
        };
        self.vscope.reverse();
        println!("WE FIND: {}", output);
        for i in &self.vscope {
            if i.vlookup.contains_key(&output) {
                {
                    println!("variable with name of: {output}, maps to : {i:?} ");
                    return Ok(());
                }
            }
        }
        println!("print scope: {:?}", self.vscope);
        println!("print output: {:?}", output);
        return self.error("this varible must be declared before use ", Some(x));
    }

    pub fn resolve_non_term(
        &mut self,
        op: &parser::Op,
        trees: &Vec<Tree>,
    ) -> Result<(), miette::Report> {
        if *op == parser::Op::Group {
            self.vscope.push(Scope::new());
        }
        for i in trees {
            self.visit_first(i)?;
        }
        if *op == parser::Op::Group {
            self.vscope.pop();
        }
        Ok(())
    }
    pub fn resolve_fun(
        &mut self,
        callee: &Rc<Tree>,
        arguments: &Vec<Tree>,
    ) -> Result<(), miette::Report> {
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
                return self.error("only characters are supported as function names: ", Some(x));
            }
        };
        let len = arguments.len();
        for i in arguments {
            self.visit_first(i)?;
        }
        match self.func_table.get(&name) {
            Some(x) => {
                let function = self.fscope.get(*x).unwrap();
                self.current_id += 1;
                if len == function.airity {
                    println!("FUNCTION: {} IS FULLY QUALIFIED", name);
                    Ok(())
                } else {
                    println!("FUNCTION: {} IS NOT FULLY QUALIFIED", name);
                    panic!("func wrong num args")
                }
            }
            None => {
                let val = format!("the function must be declared: {}", name);
                return self.error(&val, None);
            }
        }
    }
    pub fn ext_value(&mut self, input: &parser::Atom) -> Option<String> {
        let out = match input {
            parser::Atom::Ident { range, source } => {
                let val = &(*source).as_str()[range.start..range.end];
                Some(String::from(val))
            }
            _ => unreachable!(),
        };
        out
    }
    pub fn resolve(&mut self) -> Result<(), miette::Report> {
        let ast = self.ast.parse_program()?;
        println!("the AST: {ast:?}");
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
