#[allow(dead_code)]
pub mod memory {
    use std::fmt::{Debug, Write};

    use object::{Object, ObjectSection};
    use rangemap::RangeMap;

    #[derive(Clone, Copy)]
    pub struct VirtualMemory(u64);

    #[derive(Clone, Copy)]
    pub struct PhysicalMemory(u64);

    pub fn to_physical(mapping: RangeMap<u64, u64>, vm: VirtualMemory) -> Option<PhysicalMemory> {
        mapping
            .get_key_value(&vm.0)
            .map(|(virt, phys)| vm.0 - virt.start + phys)
            .map(PhysicalMemory)
    }

    pub struct Memory {
        sections: RangeMap<usize, Section>,
    }

    #[derive(Clone, PartialEq, Eq)]
    pub struct Section {
        name: String,
        block: Vec<u8>,
    }

    impl Memory {
        pub fn new() -> Self {
            Self {
                sections: RangeMap::new(),
            }
        }

        pub fn from_binary(binary_data: &[u8]) -> Self {
            let file = object::File::parse(binary_data).unwrap();

            let mut sections = RangeMap::new();

            for section in file.sections() {
                let start = section.address() as usize;
                let size = section.size() as usize;
                let end = start + size;

                let block = if let Some((file_ptr, offset)) = section.file_range() {
                    let slice = &binary_data[file_ptr as usize..(file_ptr + offset) as usize];
                    let mut vec = slice.to_vec();

                    vec.resize(size, 0);

                    vec
                } else {
                    vec![0; size]
                };

                let section = Section {
                    name: section.name().unwrap().to_string(),
                    block,
                };

                sections.insert(start..end, section);
            }

            Self { sections }
        }
    }

    impl Debug for Memory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("\n").unwrap();
            for (range, section) in self.sections.clone().into_iter() {
                f.write_fmt(format_args!("Contents of section {}:\n", section.name))
                    .unwrap();

                let mut addr = range.start;
                for data in section.block.chunks(0x10) {
                    let mut s = String::new();
                    let mut counter = 0;
                    for byte in data.into_iter() {
                        write!(&mut s, "{:02X}", byte).expect("Unable to write");
                        if counter % 4 == 3 {
                            write!(&mut s, " ").expect("Unable to write");
                        }
                        counter += 1;
                    }
                    let mut s2 = String::new();
                    for byte in data.into_iter() {
                        let c = *byte as char;
                        if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                            write!(&mut s2, "{}", c).expect("Unable to write");
                        } else {
                            write!(&mut s2, ".").expect("Unable to write");
                        }
                    }
                    f.write_fmt(format_args!("{:>5X} {:<36} {}\n", addr, s, s2))
                        .unwrap();
                    addr += 0x10;
                }
                f.write_str("\n").unwrap();
            }

            Ok(())
        }
    }
}
