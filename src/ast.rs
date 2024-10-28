use std::fmt::Display;
use std::fmt::Write;

use iced_x86::Instruction;
use iced_x86::OpKind;
use iced_x86::Register;

pub struct Expr {
    pub id: u32,
    pub kind: ExprKind,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("[EXPR #{}: {}]", &self.id, &self.kind))
        f.write_fmt(format_args!("{}", &self.kind))
    }
}

#[non_exhaustive]
pub enum ExprKind {
    // Array(ThinVec<P<Expr>>),
    // ConstBlock(AnonConst),
    Call(u64, Vec<Box<Expr>>),
    // MethodCall(Box<MethodCall>),
    // Tup(ThinVec<P<Expr>>),
    Binary(BinOpKind, Box<Expr>, Box<Expr>),
    // Unary(UnOp, P<Expr>),
    Lit(Lit),
    // Cast(P<Expr>, P<Ty>),
    // Type(P<Expr>, P<Ty>),
    // Let(P<Pat>, P<Expr>, Span, Recovered),
    // If(P<Expr>, P<Block>, Option<P<Expr>>),
    // While(P<Expr>, P<Block>, Option<Label>),
    // ForLoop {
    //     pat: P<Pat>,
    //     iter: P<Expr>,
    //     body: P<Block>,
    //     label: Option<Label>,
    //     kind: ForLoopKind,
    // },
    // Loop(P<Block>, Option<Label>, Span),
    // Match(P<Expr>, ThinVec<Arm>, MatchKind),
    // Closure(Box<Closure>),
    // Block(P<Block>, Option<Label>),
    // Gen(CaptureBy, P<Block>, GenBlockKind, Span),
    // Await(P<Expr>, Span),
    // TryBlock(P<Block>),
    Assign(Box<Expr>, Box<Expr>),
    // AssignOp(BinOp, P<Expr>, P<Expr>),
    // Field(P<Expr>, Ident),
    // Index(P<Expr>, P<Expr>, Span),
    // Range(Option<P<Expr>>, Option<P<Expr>>, RangeLimits),
    // Underscore,
    // Path(Option<P<QSelf>>, Path),
    // AddrOf(BorrowKind, Mutability, P<Expr>),
    // Break(Option<Label>, Option<P<Expr>>),
    // Continue(Option<Label>),
    // Ret(Option<P<Expr>>),
    // InlineAsm(P<InlineAsm>),
    // OffsetOf(P<Ty>, P<[Ident]>),
    // MacCall(P<MacCall>),
    // Struct(P<StructExpr>),
    // Repeat(P<Expr>, AnonConst),
    // Paren(P<Expr>),
    // Try(P<Expr>),
    // Yield(Option<P<Expr>>),
    // Yeet(Option<P<Expr>>),
    // Become(P<Expr>),
    // IncludedBytes(Lrc<[u8]>),
    // FormatArgs(P<FormatArgs>),
    // Err(ErrorGuaranteed),
    Unparsed(iced_x86::Instruction), // FIXME
}

impl Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprKind::Binary(bin_op_kind, lhs, rhs) => {
                f.write_fmt(format_args!("{} {} {}", &lhs, &bin_op_kind, &rhs))
            }
            ExprKind::Lit(lit) => f.write_fmt(format_args!("{lit}")),
            ExprKind::Assign(lhs, rhs) => f.write_fmt(format_args!("{lhs} = {rhs}")),
            ExprKind::Call(addr, _args) => f.write_fmt(format_args!("FUN_{:X}()", &addr)),
            ExprKind::Unparsed(code) => {
                f.write_fmt(format_args!("{:#?}", &code.code())).unwrap();
                match code.op0_kind() {
                    iced_x86::OpKind::Register => {
                        if code.op0_register() != iced_x86::Register::None {
                            f.write_fmt(format_args!(" {:#?}", code.op0_register()))
                                .unwrap()
                        }
                    }
                    iced_x86::OpKind::Memory => f
                        .write_fmt(format_args!(" 0x{:X}", &code.memory_displacement64()))
                        .unwrap(),
                    iced_x86::OpKind::Immediate8to64 => f
                        .write_fmt(format_args!(" {}", &code.immediate8to64()))
                        .unwrap(),
                    _ => {}
                }
                match code.op1_kind() {
                    iced_x86::OpKind::Register => {
                        if code.op1_register() != iced_x86::Register::None {
                            f.write_fmt(format_args!(" {:#?}", &code.op1_register()))
                                .unwrap()
                        }
                    }
                    iced_x86::OpKind::Memory => f
                        .write_fmt(format_args!(" 0x{:X}", &code.memory_displacement64()))
                        .unwrap(),
                    iced_x86::OpKind::Immediate8to64 => f
                        .write_fmt(format_args!(" {}", &code.immediate8to64()))
                        .unwrap(),
                    _ => {}
                }

                match code.op2_kind() {
                    iced_x86::OpKind::Register => {
                        if code.op2_register() != iced_x86::Register::None {
                            f.write_fmt(format_args!(" {:#?}", &code.op2_register()))
                                .unwrap()
                        }
                    }
                    iced_x86::OpKind::Memory => f
                        .write_fmt(format_args!(" 0x{:X}", &code.memory_displacement64()))
                        .unwrap(),
                    iced_x86::OpKind::Immediate8to64 => f
                        .write_fmt(format_args!(" {}", &code.immediate8to64()))
                        .unwrap(),
                    _ => {}
                }

                match code.op3_kind() {
                    iced_x86::OpKind::Register => {
                        if code.op3_register() != iced_x86::Register::None {
                            f.write_fmt(format_args!(" {:#?}", &code.op3_register()))
                                .unwrap()
                        }
                    }
                    iced_x86::OpKind::Memory => f
                        .write_fmt(format_args!(" 0x{:X}", &code.memory_displacement64()))
                        .unwrap(),
                    iced_x86::OpKind::Immediate8to64 => f
                        .write_fmt(format_args!(" {}", &code.immediate8to64()))
                        .unwrap(),
                    _ => {}
                }
                Ok(())
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    BitXor,
    BitAnd,
    BitOr,
    Shl,
    Shr,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
}

impl Display for BinOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOpKind::Add => f.write_str("+"),
            BinOpKind::Sub => f.write_str("-"),
            BinOpKind::Mul => f.write_str("*"),
            BinOpKind::Div => f.write_str("/"),
            BinOpKind::Rem => f.write_str("%"),
            BinOpKind::And => f.write_str("&"),
            BinOpKind::Or => f.write_str("|"),
            BinOpKind::BitXor => f.write_str("^"),
            BinOpKind::BitAnd => f.write_str("bit&"),
            BinOpKind::BitOr => f.write_str("bit|"),
            BinOpKind::Shl => f.write_str("<<"),
            BinOpKind::Shr => f.write_str(">>"),
            BinOpKind::Eq => f.write_str("=="),
            BinOpKind::Lt => f.write_str("<"),
            BinOpKind::Le => f.write_str("<="),
            BinOpKind::Ne => f.write_str("!="),
            BinOpKind::Ge => f.write_str(">="),
            BinOpKind::Gt => f.write_str(">"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Lit {
    StillUnknown,
    Symbol(Symbol),
    Bool(bool),
    U32(u32),
    I64(i64),
    Str(String),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StillUnknown => f.write_str("UNK"),
            Lit::Symbol(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Lit::Bool(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Lit::U32(arg0) => f.write_fmt(format_args!("0x{:X}", arg0)),
            Lit::Str(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Lit::I64(arg0) => f.write_fmt(format_args!("0x{:X}", arg0)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Symbol {
    Var(usize),
    Reg(String),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Var(idx) => f.write_fmt(format_args!("#{}", &idx)),
            Symbol::Reg(name) => f.write_fmt(format_args!("@{}", &name.to_uppercase())),
        }
    }
}

pub fn get_new_var_symbol(symbols: &mut Vec<Symbol>) -> Symbol {
    let idx = symbols.len();
    let sym = Symbol::Var(idx);
    symbols.push(sym.clone());
    sym
}

pub fn get_new_reg_symbol(symbols: &mut Vec<Symbol>, reg: Register) -> Symbol {
    let sym = Symbol::Reg(get_register_name(reg));
    symbols.push(sym.clone());
    sym
}

pub fn get_register_name(reg: Register) -> String {
    let mut s = String::new();
    write!(&mut s, "{:?}", reg).unwrap();
    s
}

pub fn get_new_symbol_from_inst(
    symbols: &mut Vec<Symbol>,
    inst: Instruction,
    idx: usize,
) -> Lit {
    match idx {
        0 => match inst.op0_kind() {
            OpKind::Register => return Lit::Symbol(get_new_reg_symbol(symbols, inst.op0_register())),
            _ => {}
        },
        1 => match inst.op1_kind() {
            OpKind::Register => return Lit::Symbol(get_new_reg_symbol(symbols, inst.op1_register())),
            OpKind::Immediate8to64 => return Lit::I64(inst.immediate8to64()),
            _ => {}
        },
        2 => match inst.op2_kind() {
            OpKind::Register => return Lit::Symbol(get_new_reg_symbol(symbols, inst.op2_register())),
            _ => {}
        },
        3 => match inst.op3_kind() {
            OpKind::Register => return Lit::Symbol(get_new_reg_symbol(symbols, inst.op3_register())),
            _ => {}
        },
        _ => {}
    }
    // println!("{:#?}", inst);
    return Lit::Symbol(get_new_var_symbol(symbols));
}
