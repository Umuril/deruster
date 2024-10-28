use crate::ast::*;
use iced_x86::{Code, Instruction, OpKind};

pub fn to_expression(
    _current_stack: &mut Vec<Lit>,
    symbols: &mut Vec<Symbol>,
    inst: Instruction,
) -> Expr {
    match inst.code() {
        iced_x86::Code::Push_r64 => {
            _current_stack.push(get_new_symbol_from_inst(symbols, inst, 0));
            return Expr {
                id: 0,
                kind: ExprKind::Lit(Lit::StillUnknown),
            };
        },
        iced_x86::Code::Call_rm64 => {
            if inst.op0_kind() == OpKind::Memory {
                return Expr {
                    id: 0,
                    kind: ExprKind::Call(inst.memory_displacement64(), Vec::new()),
                };
            } else {
                todo!();
            }
        }
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
                            kind: ExprKind::Lit(get_new_symbol_from_inst(
                                symbols, inst, 0,
                            )),
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
                        kind: ExprKind::Lit(get_new_symbol_from_inst(
                            symbols, inst, 0,
                        )),
                    }),
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Lit(get_new_symbol_from_inst(
                            symbols, inst, 1,
                        )),
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
                        kind: ExprKind::Lit(get_new_symbol_from_inst(
                            symbols, inst, 0,
                        )),
                    }),
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Lit(get_new_symbol_from_inst(
                            symbols, inst, 1,
                        )),
                    }),
                ),
            };
        }
        Code::And_rm64_imm8 => {
            return Expr {
                id: 0,
                kind: ExprKind::Assign(
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Lit(get_new_symbol_from_inst(symbols, inst, 0)),
                    }),
                    Box::new(Expr {
                        id: 0,
                        kind: ExprKind::Binary(
                            BinOpKind::And,
                            Box::new(Expr {
                                id: 0,
                                kind: ExprKind::Lit(get_new_symbol_from_inst(symbols, inst, 0)),
                            }),
                            Box::new(Expr {
                                id: 0,
                                kind: ExprKind::Lit(get_new_symbol_from_inst(symbols, inst, 1)),
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
