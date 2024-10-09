use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;
use iced_x86::Decoder;

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
    let binary_data = fs::read("assets/test")?;
    let file = object::File::parse(&*binary_data)?;

    let entrypoint = dbg!(file.entry());
    let data = &binary_data[(entrypoint as usize)..];

    println!("Data: {:x?}", &data[0..20]);

    let mut decoder = Decoder::with_ip(64, &data, entrypoint, 0);

    while decoder.can_decode() {
        let inst = decoder.decode();
        dbg!(inst.code());
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