use crate::scanner::*;
enum Ops {
    Add,
    Bang,
    Div,
    Mul,
    Sub,
}

// enum from : https://github.com/Darksecond/lox/blob/master/lox-syntax/src/expr_parser.rs#L15C1-L21C4
enum Precedence {
    None,
    Assign, // =
    Or,
    And,
    Equality,   // == !=
    Comparison, // < <= > >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // ()
    List,       // []
    Primary,
}

struct Parser {
    token: [TokenType],
}

//
// pub fn parse() {
// this is vaugle what it should look like
//     loop {
//         parse();
//     }
// }
