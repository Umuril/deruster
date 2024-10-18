use iced_x86::{Code, Decoder, Instruction, OpKind, Register};
use object::Object;
use std::error::Error;
use std::fmt::Display;
use std::fs;

pub struct Expr {
    pub id: u32,
    pub kind: ExprKind,
    // pub tokens: Option<LazyAttrTokenStream>,
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
    // Call(P<Expr>, ThinVec<P<Expr>>),
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
    Unparsed(Instruction),
}

impl Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprKind::Binary(bin_op_kind, lhs, rhs) => {
                f.write_fmt(format_args!("{} {} {}", &lhs, &bin_op_kind, &rhs))
            }
            ExprKind::Lit(lit) => f.write_fmt(format_args!("{lit}")),
            ExprKind::Assign(lhs, rhs) => f.write_fmt(format_args!("{lhs} = {rhs}")),
            ExprKind::Unparsed(code) => {
                f.write_fmt(format_args!("{:#?}", &code.code())).unwrap();
                match code.op0_kind() {
                    OpKind::Register => {
                        if code.op0_register() != Register::None {
                            f.write_fmt(format_args!(" {:#?}", code.op0_register()))
                                .unwrap()
                        }
                    }
                    OpKind::Memory => f
                        .write_fmt(format_args!(" 0x{:X}", &code.memory_displacement64()))
                        .unwrap(),
                    OpKind::Immediate8to64 => f
                        .write_fmt(format_args!(" {}", &code.immediate8to64()))
                        .unwrap(),
                    _ => {}
                }
                match code.op1_kind() {
                    OpKind::Register => {
                        if code.op1_register() != Register::None {
                            f.write_fmt(format_args!(" {:#?}", &code.op1_register()))
                                .unwrap()
                        }
                    }
                    OpKind::Memory => f
                        .write_fmt(format_args!(" 0x{:X}", &code.memory_displacement64()))
                        .unwrap(),
                    OpKind::Immediate8to64 => f
                        .write_fmt(format_args!(" {}", &code.immediate8to64()))
                        .unwrap(),
                    _ => {}
                }

                match code.op2_kind() {
                    OpKind::Register => {
                        if code.op2_register() != Register::None {
                            f.write_fmt(format_args!(" {:#?}", &code.op2_register()))
                                .unwrap()
                        }
                    }
                    OpKind::Memory => f
                        .write_fmt(format_args!(" 0x{:X}", &code.memory_displacement64()))
                        .unwrap(),
                    OpKind::Immediate8to64 => f
                        .write_fmt(format_args!(" {}", &code.immediate8to64()))
                        .unwrap(),
                    _ => {}
                }

                match code.op3_kind() {
                    OpKind::Register => {
                        if code.op3_register() != Register::None {
                            f.write_fmt(format_args!(" {:#?}", &code.op3_register()))
                                .unwrap()
                        }
                    }
                    OpKind::Memory => f
                        .write_fmt(format_args!(" 0x{:X}", &code.memory_displacement64()))
                        .unwrap(),
                    OpKind::Immediate8to64 => f
                        .write_fmt(format_args!(" {}", &code.immediate8to64()))
                        .unwrap(),
                    _ => {}
                }
                Ok(())
            }
        }
    }
}

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

pub enum Lit {
    StillUnknown,
    Symbol(Symbol),
    Bool(bool),
    U32(u32),
    Str(String),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StillUnknown => f.write_str("UNK"),
            Lit::Symbol(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Lit::Bool(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Lit::U32(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Lit::Str(arg0) => f.write_fmt(format_args!("{}", arg0)),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Symbol(usize);

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("SYM_{}", &self.0))
    }
}

fn get_new_symbol(symbols: &mut Vec<Symbol>) -> Symbol {
    let idx = symbols.len();
    let sym = Symbol(idx);
    symbols.push(sym);
    sym
}

fn to_expression(
    _current_stack: &mut Vec<u64>,
    symbols: &mut Vec<Symbol>,
    inst: Instruction,
) -> Expr {
    match inst.code() {
        iced_x86::Code::Xor_rm32_r32 => {
            if inst.op0_kind() == OpKind::Register
                && inst.op1_kind() == OpKind::Register
                && inst.op0_register() == inst.op1_register()
            {
                return Expr {
                    id: 0,
                    kind: ExprKind::Assign(
                        Box::new(Expr {
                            id: 0,
                            kind: ExprKind::Lit(Lit::Symbol(get_new_symbol(symbols))),
                        }),
                        Box::new(Expr {
                            id: 0,
                            kind: ExprKind::Lit(Lit::U32(0)),
                        }),
                    ),
                };
            }
            return Expr {
                id: 0,
                kind: ExprKind::Binary(
                    BinOpKind::BitXor,
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Lit(Lit::Symbol(get_new_symbol(symbols))),
                    }),
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Lit(Lit::Symbol(get_new_symbol(symbols))),
                    }),
                ),
            };
        }
        Code::Mov_rm64_r64 => {
            return Expr {
                id: 0,
                kind: ExprKind::Assign(
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Lit(Lit::Symbol(get_new_symbol(symbols))),
                    }),
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Lit(Lit::Symbol(get_new_symbol(symbols))),
                    }),
                ),
            };
        }
        Code::And_rm64_imm8 => {
            let rsp = get_new_symbol(symbols);
            return Expr {
                id: 0,
                kind: ExprKind::Assign(
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Lit(Lit::Symbol(rsp)),
                    }),
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Binary(
                            BinOpKind::And,
                            Box::new(Expr {
                                id: 0,
                                kind: ExprKind::Lit(Lit::Symbol(rsp)),
                            }),
                            Box::new(Expr {
                                id: 0,
                                kind: ExprKind::Lit(Lit::Symbol(get_new_symbol(symbols))),
                            }),
                        ),
                    }),
                ),
            };
        }
        _ => Expr {
            id: 0,
            kind: ExprKind::Unparsed(inst),
        },
    }
}

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
    let binary_data = fs::read("assets/test")?;
    let file = object::File::parse(&*binary_data)?;

    let entrypoint = dbg!(file.entry());
    let data = &binary_data[(entrypoint as usize)..];

    println!("Data: {:x?}", &data[0..20]);

    let mut decoder = Decoder::with_ip(64, &data, entrypoint, 0);

    let mut stack = Vec::new();
    let mut symbols = Vec::new();

    while decoder.can_decode() {
        let inst = decoder.decode();
        let expr = to_expression(&mut stack, &mut symbols, inst);

        if inst.code() == iced_x86::Code::Aadd_m32_r32 {
            dbg!(&inst);
            println!("0x{:X}", &inst.memory_displacement64());
        }
        println!("{}", &expr);

        if inst.code() == iced_x86::Code::Hlt {
            break;
        }
    }

    // for section in file.sections() {
    //     println!("{}", section.name()?);
    // }
    Ok(())
}

// 0000000000001040 <_start>:
//     1040:       31 ed                   xor    %ebp,%ebp
//     1042:       49 89 d1                mov    %rdx,%r9
//     1045:       5e                      pop    %rsi
//     1046:       48 89 e2                mov    %rsp,%rdx
//     1049:       48 83 e4 f0             and    $0xfffffffffffffff0,%rsp
//     104d:       50                      push   %rax
//     104e:       54                      push   %rsp
//     104f:       45 31 c0                xor    %r8d,%r8d
//     1052:       31 c9                   xor    %ecx,%ecx
//     1054:       48 8d 3d ce 00 00 00    lea    0xce(%rip),%rdi        # 1129 <main>
//     105b:       ff 15 5f 2f 00 00       call   *0x2f5f(%rip)        # 3fc0 <__libc_start_main@GLIBC_2.34>
//     1061:       f4                      hlt
