use traits::*;

/// An R-type instruction

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Rtype {
    pub funct7: u32,
    pub rs2:    u32,
    pub rs1:    u32,
    pub funct3: u32,
    pub rd:     u32,
    pub opcode: u32,
}

impl From<u32> for Rtype {
    #[inline]
    fn from(inst: u32) -> Self {
        Rtype {
            funct7: inst.extract_bitfield(25, 32),
            rs2:    inst.extract_bitfield(20, 25),
            rs1:    inst.extract_bitfield(15, 20),
            funct3: inst.extract_bitfield(12, 15),
            rd:     inst.extract_bitfield( 7, 12),
            opcode: inst.extract_bitfield(0, 7),
        }
    }
}

impl From<Rtype> for u32 {
    #[inline]
    fn from(value: Rtype) -> Self {
        value.opcode | (value.rd << 7) | (value.funct3 << 12) | (value.rs1 << 15) 
            | (value.rs2 << 20) | (value.funct7 << 25)
    }
}

/// An R4-type instruction

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct R4type {
    pub funct2: u32,
    pub rs3:    u32,
    pub rs2:    u32,
    pub rs1:    u32,
    pub funct3: u32,
    pub rd:     u32,
    pub opcode: u32,
}

impl From<u32> for R4type {
    #[inline]
    fn from(inst: u32) -> Self {
        R4type {
            rs3:    inst.extract_bitfield(27, 32),
            funct2: inst.extract_bitfield(25, 27),
            rs2:    inst.extract_bitfield(20, 25),
            rs1:    inst.extract_bitfield(15, 20),
            funct3: inst.extract_bitfield(12, 15),
            rd:     inst.extract_bitfield( 7, 12),
            opcode: inst.extract_bitfield(0, 7),
        }
    }
}

impl From<R4type> for u32 {
    #[inline]
    fn from(value: R4type) -> Self {
        value.opcode | (value.rd << 7) | (value.funct3 << 12) | (value.rs1 << 15) 
            | (value.rs2 << 20) | (value.funct2 << 25) | (value.rs3 << 27)
    }
}

/// An S-type instruction
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Stype {
    pub imm:    i32,
    pub rs2:    u32,
    pub rs1:    u32,
    pub funct3: u32,
    pub opcode: u32,
}

impl From<u32> for Stype {
    #[inline]
    fn from(inst: u32) -> Self {
        let imm115 = inst.extract_bitfield(25, 32);
        let imm40  = inst.extract_bitfield( 7, 12);

        let imm = (imm115 << 5) | imm40;

        Stype {
            imm:    imm.sign_extend(12).to_signed(),
            rs2:    inst.extract_bitfield(20, 25),
            rs1:    inst.extract_bitfield(15, 20),
            funct3: inst.extract_bitfield(12, 15),
            opcode: inst.extract_bitfield(0, 7),
        }
    }
}

impl From<Stype> for u32 {
    #[inline]
    fn from(value: Stype) -> Self {
        let imm = value.imm as u32;
        let imm_high = imm.extract_bitfield(5, 12);
        let imm_low  = imm.extract_bitfield(0, 5);
        value.opcode | (imm_low << 7) | (value.funct3 << 12) 
            | (value.rs1 << 15) | (value.rs2 << 20) | (imm_high << 25)
    }
}

/// A J-type instruction

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Jtype {
    pub imm: i32,
    pub rd:  u32,
    pub opcode: u32,
}

impl From<u32> for Jtype {
    #[inline]
    fn from(inst: u32) -> Self {
        let imm20   = inst.extract_bitfield(31, 32);
        let imm101  = inst.extract_bitfield(21, 31);
        let imm11   = inst.extract_bitfield(20, 21);
        let imm1912 = inst.extract_bitfield(12, 20);

        let imm = (imm20 << 20) | (imm1912 << 12) 
            | (imm11 << 11) | (imm101 << 1);

        Jtype {
            imm: imm.sign_extend(20).to_signed(),
            rd:  inst.extract_bitfield(7, 12),
            opcode: inst.extract_bitfield(0, 7),
        }
    }
}

impl From<Jtype> for u32 {
    #[inline]
    fn from(value: Jtype) -> Self {
        let imm = value.imm as u32;
        let imm20   = imm.extract_bitfield(20, 21);
        let imm1912 = imm.extract_bitfield(11, 20);
        let imm11   = imm.extract_bitfield(10, 11);
        let imm101  = imm.extract_bitfield(1, 11);
        value.opcode | (value.rd << 7) | (imm1912 << 12) | (imm11 << 20) 
            | (imm101 << 21) | (imm20 << 31)
    }
}

/// A B-type instruction

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Btype {
    pub imm:    i32,
    pub rs2:    u32,
    pub rs1:    u32,
    pub funct3: u32,
    pub opcode: u32,
}

impl From<u32> for Btype {
    #[inline]
    fn from(inst: u32) -> Self {
        let imm12  = inst.extract_bitfield(31, 32);
        let imm105 = inst.extract_bitfield(25, 31);
        let imm41  = inst.extract_bitfield(8, 12);
        let imm11  = inst.extract_bitfield(7, 8);

        let imm = (imm12  << 12) 
                    | (imm11  << 11) 
                    | (imm105 << 5) 
                    | (imm41  << 1);

        Btype {
            imm:    imm.sign_extend(12).to_signed(),
            rs2:    inst.extract_bitfield(20, 25),
            rs1:    inst.extract_bitfield(15, 20),
            funct3: inst.extract_bitfield(12, 15),
            opcode: inst.extract_bitfield(0, 7),
        }
    }
}

impl From<Btype> for u32 {
    #[inline]
    fn from(value: Btype) -> Self {
        let imm = value.imm as u32;
        let imm_4_1 = imm.extract_bitfield(1, 5);
        let imm_11 = imm.extract_bitfield(10, 11);
        let imm_10_5 = imm.extract_bitfield(5, 11);
        let imm_12 = imm.extract_bitfield(11, 12);

        value.opcode | (imm_11 << 7) | (imm_4_1 << 8) | (value.funct3 << 12) 
            | (value.rs1 << 15) | (value.rs2 << 20) 
            | (imm_10_5 << 25) | (imm_12 << 31)
    
    }
}

/// An I-type instruction

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Itype {
    pub imm:    i32,
    pub rs1:    u32,
    pub funct3: u32,
    pub rd:     u32,
    pub opcode: u32,
}

impl From<u32> for Itype {
    #[inline]
    fn from(inst: u32) -> Self {
        let imm = (inst as i32) >> 20; // TODO! check
        Itype {
            imm:    imm,
            rs1:    inst.extract_bitfield(15, 20),
            funct3: inst.extract_bitfield(12, 15),
            rd:     inst.extract_bitfield(7, 12),
            opcode: inst.extract_bitfield(0, 7),
        }
    }
}

impl From<Itype> for u32 {
    #[inline]
    fn from(value: Itype) -> Self {
        value.opcode | (value.rd << 7) | (value.funct3 << 12) 
            | (value.rs1 << 15) | ((value.imm as u32) << 20)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Utype {
    pub imm: u32,
    pub rd:  u32,
    pub opcode: u32,
}

impl From<u32> for Utype {
    #[inline]
    fn from(inst: u32) -> Self {
        Utype {
            imm: inst.extract_bitfield(12, 32),
            rd:  inst.extract_bitfield(7, 12),
            opcode: inst.extract_bitfield(0, 7),
        }
    }
}

impl From<Utype> for u32 {
    #[inline]
    fn from(value: Utype) -> Self {
        value.opcode | (value.rd << 7) | (value.imm << 12)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct CRtype {
    pub funct4: u16,
    pub rd_rs1: u16,
    pub rs2:    u16,
    pub opcode: u16,
}

impl From<u16> for CRtype {
    #[inline]
    fn from(inst: u16) -> Self {
        CRtype {
            funct4: inst.extract_bitfield(12, 16),
            rd_rs1: inst.extract_bitfield( 7, 12),
            rs2:    inst.extract_bitfield( 2,  7),
            opcode:    inst.extract_bitfield( 0,  2),
        }
    }
}

impl From<CRtype> for u16 {
    #[inline]
    fn from(value: CRtype) -> Self {
        value.opcode | (value.rs2 << 2) | (value.rd_rs1 << 7) 
            | (value.funct4 << 12)
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct CItype {
    pub funct3: u16,
    pub imm2:   u16,
    pub rd_rs1: u16,
    pub imm1:   u16,
    pub opcode: u16,
}

impl From<u16> for CItype {
    #[inline]
    fn from(inst: u16) -> Self {
        CItype {
            funct3: inst.extract_bitfield(13, 16),
            imm2:   inst.extract_bitfield(12, 13),
            rd_rs1: inst.extract_bitfield( 7, 12),
            imm1:   inst.extract_bitfield( 2,  7),
            opcode:    inst.extract_bitfield( 0,  2),
        }
    }
}

impl From<CItype> for u16 {
    #[inline]
    fn from(value: CItype) -> Self {
        value.opcode | (value.imm1 << 2) | (value.rd_rs1 << 7) 
            | (value.imm2 << 12) | (value.funct3 << 13)
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct CSStype {
    pub funct3: u16,
    pub imm:    u16,
    pub rs2:    u16,
    pub opcode: u16,
}

impl From<u16> for CSStype {
    #[inline]
    fn from(inst: u16) -> Self {
        CSStype {
            funct3: inst.extract_bitfield(13, 16),
            imm:    inst.extract_bitfield( 7, 13),
            rs2:    inst.extract_bitfield( 2,  7),
            opcode:    inst.extract_bitfield( 0,  2),
        }
    }
}

impl From<CSStype> for u16 {
    #[inline]
    fn from(value: CSStype) -> Self {
        value.opcode | (value.rs2 << 2) | (value.imm << 7) 
            | (value.funct3 << 13)
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct CIWtype {
    pub funct3:   u16,
    pub imm:      u16,
    pub rd_prime: u16,
    pub opcode: u16,
}

impl From<u16> for CIWtype {
    #[inline]
    fn from(inst: u16) -> Self {
        CIWtype {
            funct3:   inst.extract_bitfield(13, 16),
            imm:      inst.extract_bitfield( 5, 13),
            rd_prime: inst.extract_bitfield( 2,  5),
            opcode:    inst.extract_bitfield( 0,  2),
        }
    }
}

impl From<CIWtype> for u16 {
    #[inline]
    fn from(value: CIWtype) -> Self {
        value.opcode | (value.rd_prime << 2) | (value.imm << 5) 
            | (value.funct3 << 13)
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct CLtype {
    pub funct3:    u16,
    pub imm2:      u16,
    pub rs1_prime: u16,
    pub imm1:      u16,
    pub rd_prime:  u16,
    pub opcode: u16,
}

impl From<u16> for CLtype {
    #[inline]
    fn from(inst: u16) -> Self {
        CLtype {
            funct3:    inst.extract_bitfield(13, 16),
            imm2:      inst.extract_bitfield(10, 13),
            rs1_prime: inst.extract_bitfield( 7, 10),
            imm1:      inst.extract_bitfield( 5,  7),
            rd_prime:  inst.extract_bitfield( 2,  5),
            opcode:    inst.extract_bitfield( 0,  2),
        }
    }
}

impl From<CLtype> for u16 {
    #[inline]
    fn from(value: CLtype) -> Self {
        value.opcode | (value.rd_prime << 2) | (value.imm1 << 5) 
            | (value.rs1_prime << 7) | (value.imm2 << 10) | (value.funct3 << 13)
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct CStype {
    pub funct3:    u16,
    pub imm2:      u16,
    pub rs1_prime: u16,
    pub imm1:      u16,
    pub rs2_prime:  u16,
    pub opcode: u16,
}

impl From<u16> for CStype {
    #[inline]
    fn from(inst: u16) -> Self {
        CStype {
            funct3:     inst.extract_bitfield(13, 16),
            imm2:       inst.extract_bitfield(10, 13),
            rs1_prime:  inst.extract_bitfield( 7, 10),
            imm1:       inst.extract_bitfield( 5,  7),
            rs2_prime:  inst.extract_bitfield( 2,  5),
            opcode:    inst.extract_bitfield( 0,  2),
        }
    }
}

impl From<CStype> for u16 {
    #[inline]
    fn from(value: CStype) -> Self {
        value.opcode | (value.rs2_prime << 2) | (value.imm1 << 5) 
            | (value.rs1_prime << 7) | (value.imm2 << 10) 
            | (value.funct3 << 13)
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct CAtype {
    pub funct6:       u16,
    pub rd_rs1_prime: u16,
    pub funct2:       u16,
    pub rs2_prime:    u16,
    pub opcode: u16,
}

impl From<u16> for CAtype {
    #[inline]
    fn from(inst: u16) -> Self {
        CAtype {
            funct6:       inst.extract_bitfield(10, 16),
            rd_rs1_prime: inst.extract_bitfield( 7, 10),
            funct2:       inst.extract_bitfield( 5,  7),
            rs2_prime:    inst.extract_bitfield( 2,  5),
            opcode:    inst.extract_bitfield( 0,  2),
        }
    }
}

impl From<CAtype> for u16 {
    #[inline]
    fn from(value: CAtype) -> Self {
        value.opcode | (value.rs2_prime << 2) | (value.funct2 << 5) 
            | (value.rd_rs1_prime << 7) | (value.funct6 << 10)
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct CBtype {
    pub funct3:    u16,
    pub offset2:   u16,
    pub rs1_prime: u16,
    pub offset1:   u16,
    pub opcode: u16,
}

impl From<u16> for CBtype {
    #[inline]
    fn from(inst: u16) -> Self {
        CBtype {
            funct3:    inst.extract_bitfield(13, 16),
            offset2:   inst.extract_bitfield(10, 13),
            rs1_prime: inst.extract_bitfield( 7, 10),
            offset1:   inst.extract_bitfield( 2,  7),
            opcode:    inst.extract_bitfield( 0,  2),
        }
    }
}

impl From<CBtype> for u16 {
    #[inline]
    fn from(value: CBtype) -> Self {
        value.opcode | (value.offset1 << 2) | (value.rs1_prime << 7) 
            | (value.offset2 << 10) | (value.funct3 << 13)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct CJtype {
    pub funct3:      u16,
    pub jump_target: i16,
    pub opcode: u16,
}

impl From<u16> for CJtype {
    #[inline]
    fn from(inst: u16) -> Self {
        CJtype {
            funct3:      inst.extract_bitfield(13, 16),
            jump_target: inst.extract_bitfield( 2, 13).sign_extend(10).to_signed(),
            opcode:    inst.extract_bitfield( 0,  2),
        }
    }
}

impl From<CJtype> for u16 {
    #[inline]
    fn from(value: CJtype) -> Self {
        let jmp = (value.jump_target as u16).extract_bitfield(0, 11);
        value.opcode | (jmp << 2) | (value.funct3 << 13)
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rtype() {
        let v = Rtype {
            rs1: 1,
            rs2: 2,
            rd: 3,
            funct3: 0b111,
            funct7:  0b111_111,
            opcode: 0b0110111,
        };
        let conv = Rtype::from(u32::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_r4type() {
        let v = R4type {
            rs1: 1,
            rs2: 2,
            rs3: 3,
            rd: 3,
            funct3: 0b111,
            funct2:  0b11,
            opcode: 0b0110111,
        };
        let conv = R4type::from(u32::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_stype() {
        let v = Stype {
            rs1: 1,
            rs2: 2,
            funct3: 0b_111,
            imm: -2,
            opcode: 0b0110111,
        };
        let conv = Stype::from(u32::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_jtype() {
        let v = Jtype {
            rd: 1,
            imm: -2,
            opcode: 0b0110111,
        };
        let conv = Jtype::from(u32::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_btype() {
        let v = Btype {
            rs1: 1,
            rs2: 2,
            funct3: 6,
            imm: -2,
            opcode: 0b0110111,
        };
        let conv = Btype::from(u32::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_itype() {
        let v = Itype {
            rs1: 1,
            rd: 2,
            funct3: 6,
            imm: -2,
            opcode: 0b0110111,
        };
        let conv = Itype::from(u32::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_utype() {
        let v = Utype {
            rd: 2,
            imm: 8,
            opcode: 0b0110111,
        };
        let conv = Utype::from(u32::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_citype() {
        let v = CItype {
            rd_rs1: 2,
            imm1: 31,
            imm2: 1,
            funct3: 6,
            opcode: 0b11,
        };
        let conv = CItype::from(u16::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_csstype() {
        let v = CSStype {
            rs2: 2,
            imm: 8,
            funct3: 6,
            opcode: 0b11,
        };
        let conv = CSStype::from(u16::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_ciwtype() {
        let v = CIWtype {
            rd_prime: 2,
            imm: 8,
            funct3: 6,
            opcode: 0b11,
        };
        let conv = CIWtype::from(u16::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_cltype() {
        let v = CLtype {
            rd_prime: 2,
            rs1_prime: 3,
            imm1: 3,
            funct3: 6,
            imm2: 1,
            opcode: 0b11,
        };
        let conv = CLtype::from(u16::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_cstype() {
        let v = CStype {
            rs2_prime: 2,
            rs1_prime: 3,
            imm1: 3,
            funct3: 6,
            imm2: 1,
            opcode: 0b11,
        };
        let conv = CStype::from(u16::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_catype() {
        let v = CAtype {
            rs2_prime: 2,
            rd_rs1_prime: 3,
            funct2: 3,
            funct6: 37,
            opcode: 0b11,
        };
        let conv = CAtype::from(u16::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_cbtype() {
        let v = CBtype {
            funct3: 2,
            rs1_prime: 3,
            offset2: 4,
            offset1: 5,
            opcode: 0b11,
        };
        let conv = CBtype::from(u16::from(v));
        assert_eq!(v, conv);
    }

    #[test]
    fn test_cjtype() {
        let v = CJtype {
            funct3: 2,
            jump_target: -2,
            opcode: 0b11,
        };
        let conv = CJtype::from(u16::from(v));
        assert_eq!(v, conv);
    }
}