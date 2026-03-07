// //NOTE: Right now this file is just for ideas.
// enum OpCode {
//     LoadConst,
//     Add,
//     Sub,
//     Mul,
//     Div,
//     Return,
// }
// // enum InstructionSet {
// //     Mov,
// //     Ld,
// //     Add,
// //     Sub,
// //     Str,
// //     Div,
// //     Ret,
// //     Jmp,
// // }
// // LOAD a
// // LOAD b
// // LOAD c
// // MUL
// // ADD
// //
// // Execution:
// //
// // stack = []
// //
// // LOAD a   -> push(a)
// // LOAD b   -> push(b)
// // LOAD c   -> push(c)
// //
// // MUL      -> pop c, pop b, push(b*c)
// //
// // ADD      -> pop result, pop a, push(a + result)
//
// struct JumpTable {
//     add: u8,
// }
//
// impl JumpTable {
//     pub fn new() -> Self {
//         Self { add: 0 << 1 }
//     }
// }
//
// pub struct Vm {
//     stack: Vec<u8>,
//     pc: u8,
// }
//
// impl Vm {
//     pub fn new() -> Vm {
//         Self {
//             stack: Vec::new(),
//             pc: 0,
//         }
//     }
//     pub fn write(&mut self) {}
//
//     pub fn read() {}
// }
//
// impl InstructionSet {
//     pub fn decode(&self, req: JumpTable) -> u8 {
//         match self {
//             InstructionSet::Add => req.add,
//             _ => unimplemented!(), // InstructionSet::Sub(x) => todo!(),
//         }
//     }
//
//     pub fn write() {}
//     pub fn read() {}
// }
