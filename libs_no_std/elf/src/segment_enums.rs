#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum SegmentType {
    /// The array element is unused; other members' values are undefined. 
    /// This type lets the program header table have ignored entries.
    PT_NULL,

    /// The array element specifies a loadable segment, described by p_filesz 
    /// and p_memsz. The bytes from the file are mapped to the beginning of the 
    /// memory segment. If the segment's memory size (p_memsz) is larger than 
    /// the file size (p_filesz), the ``extra'' bytes are defined to hold the 
    /// value 0 and to follow the segment's initialized area. The file size may 
    /// not be larger than the memory size. Loadable segment entries in the 
    /// program header table appear in ascending order, sorted on the p_vaddr 
    /// member.
    PT_LOAD,

    /// The array element specifies dynamic linking information.
    PT_DYNAMIC,

    /// The array element specifies the location and size of a null-terminated 
    /// path name to invoke as an interpreter. This segment type is meaningful 
    /// only for executable files (though it may occur for shared objects); 
    /// it may not occur more than once in a file. If it is present, it must 
    /// precede any loadable segment entry. 
    PT_INTERP,

    /// The array element specifies the location and size of auxiliary 
    /// information.
    PT_NOTE,

    /// This segment type is reserved but has unspecified semantics. Programs 
    /// that contain an array element of this type do not conform to the ABI.
    PT_SHLIB,

    /// The array element, if present, specifies the location and size of the 
    /// program header table itself, both in the file and in the memory image 
    /// of the program. This segment type may not occur more than once in a file.
    /// Moreover, it may occur only if the program header table is part of the 
    /// memory image of the program. If it is present, it must precede any 
    /// loadable segment entry.
    PT_PHDR,

    /// The array element specifies the Thread-Local Storage template. 
    /// Implementations need not support this program table entry
    PT_TLS,

    /// Values in this inclusive range are reserved for operating 
    /// system-specific semantics.
    PT_OS(u32),

    /// Values in this inclusive range are reserved for processor-specific 
    /// semantics. If meanings are specified, the processor supplement 
    /// explains them.
    PT_PROC(u32),

    /// Unknown type
    UNKNOWN(u32),
}

impl From<u32> for SegmentType {
    fn from(item: u32) -> Self {
        match item {
            0      => SegmentType::PT_NULL,
            1      => SegmentType::PT_LOAD,
            2      => SegmentType::PT_DYNAMIC,
            3      => SegmentType::PT_INTERP,
            4      => SegmentType::PT_NOTE,
            5      => SegmentType::PT_SHLIB,
            6      => SegmentType::PT_PHDR,
            7      => SegmentType::PT_TLS,
            0x60000000..=0x6fffffff => SegmentType::PT_OS(item),
            0x70000000..=0x7fffffff => SegmentType::PT_PROC(item),
            _ => SegmentType::UNKNOWN(item),
        }
    }
}

impl From<SegmentType> for u32 {
    fn from(item: SegmentType) -> Self {
        match item {
            SegmentType::PT_NULL    => 0,
            SegmentType::PT_LOAD    => 1,
            SegmentType::PT_DYNAMIC => 2,
            SegmentType::PT_INTERP  => 3,
            SegmentType::PT_NOTE    => 4,
            SegmentType::PT_SHLIB   => 5,
            SegmentType::PT_PHDR    => 6,
            SegmentType::PT_TLS     => 7,
            SegmentType::PT_OS(val)   => val,
            SegmentType::PT_PROC(val) => val,
            SegmentType::UNKNOWN(val) => val,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum SegmentFlags {
    /// Permissio to execute
    PT_X,

    /// Permission to write
    PT_W,

    /// Permission to read
    PT_R,

    /// 0x0ff00000 Bits reserved to operative system-specific semantics
    PT_OS(u32),

    /// 0xf0000000 Bits reserved to processor-specific semantics
    PT_PROC(u32),

    /// Values in this inclusive range are reserved for processor-specific 
    /// semantics. If meanings are specified, the processor supplement 
    /// explains them.
    PT_MULTIFLAGS(Vec<SegmentFlags>),

    /// No perimssions
    PT_NO,

    /// Unknown type
    UNKNOWN(u32),
}

impl SegmentFlags {
    /// Convert an u32 to the vector of setted flags.
    pub fn to_flags(mut val: u32) -> Vec<SegmentFlags> {
        let mut result = Vec::new();

        for flag in &[
            SegmentFlags::PT_X,
            SegmentFlags::PT_W,
            SegmentFlags::PT_R
        ] {
            let flag_num = u32::from(flag.clone());
            if 0 != (val & flag_num) {
                result.push(flag.clone());
                val ^= flag_num;
            }    
        }

        if 0 != (val & 0x0ff00000) {
            result.push(
                SegmentFlags::PT_OS(val & 0x0ff00000)
            );
            val &= !0x0ff00000;
        }

        if 0 != (val & 0xf0000000) {
            result.push(
                SegmentFlags::PT_PROC(val & 0xf0000000)
            );
            val &= !0xf0000000;
        }
        
        if val != 0 {
            result.push(
                SegmentFlags::UNKNOWN(val)
            );
        }

        if result.is_empty() {
            result.push(SegmentFlags::PT_NO);
        }

        result
    }
}

impl From<u32> for SegmentFlags {
    fn from(item: u32) -> Self {
        let flags = SegmentFlags::to_flags(item);

        if flags.len() == 1 {
            flags[0].clone()
        } else {
            SegmentFlags::PT_MULTIFLAGS(flags)
        }
    }
}

impl From<SegmentFlags> for u32 {
    fn from(item: SegmentFlags) -> Self {
        match item {
            SegmentFlags::PT_NO            =>  0x0,
            SegmentFlags::PT_X             =>  0x1,
            SegmentFlags::PT_W             =>  0x2,
            SegmentFlags::PT_R             =>  0x4,
            SegmentFlags::PT_OS(val)   => val,
            SegmentFlags::PT_PROC(val) => val,
            SegmentFlags::UNKNOWN(val)     => val,
            SegmentFlags::PT_MULTIFLAGS(val) => {
                let mut result = 0;
                for flag in val {
                    result |= u32::from(flag);
                }
                result
            },
        }
    }
}