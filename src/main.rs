use crate::parser::to_expression;
use ast::{Lit, Symbol};
use iced_x86::Decoder;
use memory::memory::Memory;
use object::Object;
use rangemap::RangeMap;
use std::error::Error;
use std::fs;
mod ast;
mod parser;

mod memory;

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
    let binary_data = fs::read("assets/test")?;
    let file = object::File::parse(&*binary_data)?;

    let entrypoint = file.entry();
    println!("Entrypoint: 0x{:X}", entrypoint);

    let data = &binary_data[(entrypoint as usize)..];

    println!("Data: {:x?} 0x{:x}", &data[0..20], binary_data.len());

    let mut functions = Vec::<u64>::new();
    let mut stack = Vec::<Lit>::new();
    let mut symbols = Vec::<Symbol>::new();

    let _vm_mappings = RangeMap::<u64, u64>::new();

    let _memory = Memory::from_binary(&binary_data);
    // dbg!(&memory);

    let _sect = file.section_by_name(".eh_frame");
    // dbg!(sect);

    // functions.push(file.entry());
    functions.push(0x1161u64);

    // dbg!(&file.symbol_map());
    // dbg!(&file.symbol_table());
    // for sym in file.symbols() {
    //     dbg!(&sym);
    // }
    // for sym in file.dynamic_symbols() {
    //     dbg!(&sym);
    // }
    // dbg!(&file.symbols());
    // dbg!(&file.dynamic_symbol_table());
    // dbg!(&file.dynamic_symbols());
    // dbg!(&file.has_debug_symbols());

    while let Some(func_start) = functions.pop() {
        println!("\nDebugging function at 0x{:X}", &func_start);

        let mut decoder =
            Decoder::with_ip(64, &binary_data[(func_start as usize)..], func_start, 0);

        while decoder.can_decode() {
            let inst = decoder.decode();
            let expr = to_expression(&mut stack, &mut symbols, inst);

            if inst.code() == iced_x86::Code::Aadd_m32_r32 {
                dbg!(&inst);
                println!("0x{:X}", &inst.memory_displacement64());
            }
            println!("{} {:?}", &expr, &stack);

            // if let ast::ExprKind::Call(func, _) = expr.kind {
            //     functions.push(func);
            // }

            if inst.code() == iced_x86::Code::Retnq {
                break;
            }

            if inst.code() == iced_x86::Code::Hlt {
                break;
            }
        }
    }

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

// void processEntry _start(undefined8 param_1,undefined8 param_2)

// {
//   undefined auStack_8 [8];
//
//   __libc_start_main(main,param_2,&stack0x00000008,0,0,param_1,auStack_8);
//   do {
//                     /* WARNING: Do nothing block with infinite loop */
//   } while( true );
// }
