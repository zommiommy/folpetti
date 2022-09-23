use bitfields::*;


/// An R-type instruction
#[derive(Debug)]
pub(crate) struct Rtype {
    pub funct7: u32,
    pub rs2:    u32,
    pub rs1:    u32,
    pub funct3: u32,
    pub rd:     u32,
}

impl From<u32> for Rtype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        Rtype {
            funct7: inst.extract_bitfield::<25, 32>(),
            rs2:    inst.extract_bitfield::<20, 25>(),
            rs1:    inst.extract_bitfield::<15, 20>(),
            funct3: inst.extract_bitfield::<12, 15>(),
            rd:     inst.extract_bitfield::< 7, 12>(),
        }
    }
}

/// An R4-type instruction
#[derive(Debug)]
pub(crate) struct R4type {
    pub funct2: u32,
    pub rs3:    u32,
    pub rs2:    u32,
    pub rs1:    u32,
    pub funct3: u32,
    pub rd:     u32,
}

impl From<u32> for R4type {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        R4type {
            rs3:    inst.extract_bitfield::<27, 32>(),
            funct2: inst.extract_bitfield::<25, 27>(),
            rs2:    inst.extract_bitfield::<20, 27>(),
            rs1:    inst.extract_bitfield::<15, 20>(),
            funct3: inst.extract_bitfield::<12, 15>(),
            rd:     inst.extract_bitfield::< 7, 12>(),
        }
    }
}

/// An S-type instruction
#[derive(Debug)]
pub(crate) struct Stype {
    pub imm:    i32,
    pub rs2:    u32,
    pub rs1:    u32,
    pub funct3: u32,
}

impl From<u32> for Stype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        let imm115 = inst.extract_bitfield::<25, 32>();
        let imm40  = inst.extract_bitfield::< 7, 12>();

        let imm = (imm115 << 5) | imm40;

        Stype {
            imm:    imm.sign_extend::<20>(),
            rs2:    inst.extract_bitfield::<20, 25>(),
            rs1:    inst.extract_bitfield::<15, 20>(),
            funct3: inst.extract_bitfield::<12, 15>(),
        }
    }
}

/// A J-type instruction
#[derive(Debug)]
pub(crate) struct Jtype {
    pub imm: i32,
    pub rd:  u32,
}

impl From<u32> for Jtype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        let imm20   = inst.extract_bitfield::<31, 32>();
        let imm101  = inst.extract_bitfield::<21, 31>();
        let imm11   = inst.extract_bitfield::<20, 21>();
        let imm1912 = inst.extract_bitfield::<12, 20>();

        let imm = (imm20   << 20) 
                    | (imm1912 << 12) 
                    | (imm11   << 11) 
                    | (imm101  << 1);

        Jtype {
            imm: imm.sign_extend::<11>(),
            rd:  inst.extract_bitfield::<7, 12>(),
        }
    }
}

/// A B-type instruction
#[derive(Debug)]
pub(crate) struct Btype {
    pub imm:    i32,
    pub rs2:    u32,
    pub rs1:    u32,
    pub funct3: u32,
}

impl From<u32> for Btype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        let imm12  = inst.extract_bitfield::<31, 32>();
        let imm105 = inst.extract_bitfield::<25, 31>();
        let imm41  = inst.extract_bitfield::<8, 12>();
        let imm11  = inst.extract_bitfield::<7, 8>();

        let imm = (imm12  << 12) 
                    | (imm11  << 11) 
                    | (imm105 << 5) 
                    | (imm41  << 1);

        Btype {
            imm:    imm.sign_extend::<19>(),
            rs2:    inst.extract_bitfield::<20, 25>(),
            rs1:    inst.extract_bitfield::<15, 20>(),
            funct3: inst.extract_bitfield::<12, 15>(),
        }
    }
}

/// An I-type instruction
#[derive(Debug)]
pub(crate) struct Itype {
    pub imm:    i32,
    pub rs1:    u32,
    pub funct3: u32,
    pub rd:     u32,
}

impl From<u32> for Itype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        let imm = (inst as i32) >> 20; // TODO! check
        Itype {
            imm:    imm,
            rs1:    inst.extract_bitfield::<15, 20>(),
            funct3: inst.extract_bitfield::<12, 15>(),
            rd:     inst.extract_bitfield::<7, 12>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Utype {
    pub imm: u32,
    pub rd:  u32,
}

impl From<u32> for Utype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        Utype {
            imm: inst.extract_bitfield::<12, 32>(),
            rd:  inst.extract_bitfield::<7, 12>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CRtype {
    pub funct4: u16,
    pub rd_rs1: u16,
    pub rs2:    u16,
}

impl From<u16> for CRtype {
    fn from(inst: u16) -> Self {
        debug_assert_ne!(inst & 0b11, 0b11);
        CRtype {
            funct4: inst.extract_bitfield::<12, 16>(),
            rd_rs1: inst.extract_bitfield::< 7, 12>(),
            rs2:    inst.extract_bitfield::< 2,  7>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CItype {
    pub funct3: u16,
    pub imm2:   u16,
    pub rd_rs1: u16,
    pub imm1:   u16,
}

impl From<u16> for CItype {
    fn from(inst: u16) -> Self {
        debug_assert_ne!(inst & 0b11, 0b11);
        CItype {
            funct3: inst.extract_bitfield::<13, 16>(),
            imm2:   inst.extract_bitfield::<12, 13>(),
            rd_rs1: inst.extract_bitfield::< 7, 12>(),
            imm1:   inst.extract_bitfield::< 2,  7>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CSStype {
    pub funct3: u16,
    pub imm:    u16,
    pub rs2:    u16,
}

impl From<u16> for CSStype {
    fn from(inst: u16) -> Self {
        debug_assert_ne!(inst & 0b11, 0b11);
        CSStype {
            funct3: inst.extract_bitfield::<13, 16>(),
            imm:    inst.extract_bitfield::< 7, 13>(),
            rs2:    inst.extract_bitfield::< 2,  7>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CIWtype {
    pub funct3:   u16,
    pub imm:      u16,
    pub rd_prime: u16,
}

impl From<u16> for CIWtype {
    fn from(inst: u16) -> Self {
        debug_assert_ne!(inst & 0b11, 0b11);
        CIWtype {
            funct3:   inst.extract_bitfield::<13, 16>(),
            imm:      inst.extract_bitfield::< 5, 13>(),
            rd_prime: inst.extract_bitfield::< 2,  5>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CLtype {
    pub funct3:    u16,
    pub imm2:      u16,
    pub rs1_prime: u16,
    pub imm1:      u16,
    pub rd_prime:  u16,
}

impl From<u16> for CLtype {
    fn from(inst: u16) -> Self {
        debug_assert_ne!(inst & 0b11, 0b11);
        CLtype {
            funct3:    inst.extract_bitfield::<13, 16>(),
            imm2:      inst.extract_bitfield::<10, 13>(),
            rs1_prime: inst.extract_bitfield::< 7, 10>(),
            imm1:      inst.extract_bitfield::< 5,  7>(),
            rd_prime:  inst.extract_bitfield::< 2,  5>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CStype {
    pub funct3:    u16,
    pub imm2:      u16,
    pub rs1_prime: u16,
    pub imm1:      u16,
    pub rs2_prime:  u16,
}

impl From<u16> for CStype {
    fn from(inst: u16) -> Self {
        debug_assert_ne!(inst & 0b11, 0b11);
        CStype {
            funct3:     inst.extract_bitfield::<13, 16>(),
            imm2:       inst.extract_bitfield::<10, 13>(),
            rs1_prime:  inst.extract_bitfield::< 7, 10>(),
            imm1:       inst.extract_bitfield::< 5,  7>(),
            rs2_prime:  inst.extract_bitfield::< 2,  5>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CAtype {
    pub funct6:       u16,
    pub rd_rs1_prime: u16,
    pub funct2:       u16,
    pub rs2_prime:    u16,
}

impl From<u16> for CAtype {
    fn from(inst: u16) -> Self {
        debug_assert_ne!(inst & 0b11, 0b11);
        CAtype {
            funct6:       inst.extract_bitfield::<10, 16>(),
            rd_rs1_prime: inst.extract_bitfield::< 7, 10>(),
            funct2:       inst.extract_bitfield::< 5,  7>(),
            rs2_prime:    inst.extract_bitfield::< 2,  5>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CBtype {
    pub funct3:    u16,
    pub offset2:   u16,
    pub rs1_prime: u16,
    pub offset1:   u16,
}

impl From<u16> for CBtype {
    fn from(inst: u16) -> Self {
        debug_assert_ne!(inst & 0b11, 0b11);
        CBtype {
            funct3:    inst.extract_bitfield::<13, 16>(),
            offset2:   inst.extract_bitfield::<10, 13>(),
            rs1_prime: inst.extract_bitfield::< 7, 10>(),
            offset1:   inst.extract_bitfield::< 2,  7>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct CJtype {
    pub funct3:      u16,
    pub jump_target: u16,
}

impl From<u16> for CJtype {
    fn from(inst: u16) -> Self {
        debug_assert_ne!(inst & 0b11, 0b11);
        CJtype {
            funct3:      inst.extract_bitfield::<13, 16>(),
            jump_target: inst.extract_bitfield::< 2, 13>(),
        }
    }
}

/// Helper function to build compact integers
pub(crate) fn compose_imms_53_76(imm1: u16, imm2: u16) -> u16 {
    (imm1 << 6) | (imm2 << 3)
}

/// Helper function to build compact integers
pub(crate) fn compose_imms_53_2_or_6(imm1: u16, imm2: u16) -> u16 {
    ((imm1 & 0b1) << 6) | (imm2 << 3) | (imm1 & 0b10) 
}
