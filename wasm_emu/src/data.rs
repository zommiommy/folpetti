use super::*;

#[derive(Debug, Clone, PartialEq)]
/// A passive data segment’s contents can be copied into a memory using the 
/// memory.init instruction. An active data segment copies its contents into 
/// a memory during instantiation, as specified by a memory index and a 
/// constant expression defining an offset into that memory.
pub enum Data {
    Active{
        /// The optional index, by default is 0, this is the "segment of memory" 
        /// where the data shall be loaded.
        /// In the current version of WebAssembly, at most one memory may be defined 
        /// or imported in a single module, so all valid active data segments have 
        /// a memory value of 0.
        memory_idx: MemIdx,

        /// The "address" inside the segment where the data will be loaded.
        offset: Expression,

        /// The content of the data section.
        bytes: Vec<u8>,
    },
    Passive {
        /// The content of the data section.
        bytes: Vec<u8>,
    }
}

impl<'a> Parse<'a> for Data {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, mode) = get_field!(data);

        match mode {
            // active mode with memory idx = 0
            0x00 => {
                let (data, offset) = Expression::parse(data);
                let (data, bytes) = Vec::parse(data);
                (   
                    data,
                    Data::Active{
                        memory_idx: 0,
                        offset: offset,
                        bytes: bytes,
                    }
                )
            },

            // passive mode
            0x01 => {
                let (data, bytes) = Vec::parse(data);
                (   
                    data,
                    Data::Passive{
                        bytes: bytes,
                    }
                )
            },

            // active mode with explicit memory idx (this should almost never
            // be generated or used)
            0x02 => {
                let (data, memory_idx) = MemIdx::parse(data);
                let (data, offset) = Expression::parse(data);
                let (data, bytes) = Vec::parse(data);
                (   
                    data,
                    Data::Active{
                        memory_idx: memory_idx,
                        offset: offset,
                        bytes: bytes,
                    }
                )
            },
            _ => panic!("Cannot parse {} as a data mode.", mode),
        }
    }
}
