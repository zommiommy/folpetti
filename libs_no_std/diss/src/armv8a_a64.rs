use crate::utils::*;

pub enum RegA64 {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    Fp, // X29
    Lr, // x30
}

impl From<u32> for RegA64 {
    fn from(value: u32) -> Self {
        use RegA64::*;
        match value { 
            0 => X0,
            1 => X1,
            2 => X2,
            3 => X3,
            4 => X4,
            5 => X5,
            6 => X6,
            7 => X7,
            8 => X8,
            9 => X9,
           10 => X10,
           11 => X11,
           12 => X12,
           13 => X13,
           14 => X14,
           15 => X15,
           16 => X16,
           17 => X17,
           18 => X18,
           19 => X19,
           20 => X20,
           21 => X21,
           22 => X22,
           23 => X23,
           24 => X24,
           25 => X25,
           26 => X26,
           27 => X27,
           28 => X28,
           29 => Fp,
           30 => Lr,
           _ => {
               panic!("invalid register decoding {}", value);
           }
        }
    }
}

pub enum ErrorDissArmV8aA64<E> {
    UnimplementedInstruction(u32),
    UnallocatedInstruction(u32),
    UserError(E),
}

pub trait ArmV8aA64User {
    type Error;

    fn adr(&mut self, rd: RegA64, imm: u32) -> Result<(), Self::Error>;
    fn adrp(&mut self, rd: RegA64, imm: u32) -> Result<(), Self::Error>;

    fn add_imm_32(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    fn adds_imm_32(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    fn sub_imm_32(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    fn subs_imm_32(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    fn add_imm_64(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    fn adds_imm_64(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    fn sub_imm_64(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    fn subs_imm_64(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;

    fn and_imm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn orr_imm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn eor_imm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn ands_imm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn and_imm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn orr_imm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn eor_imm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn ands_imm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;

    fn movn_32(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;
    fn movz_32(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;
    fn movk_32(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;
    fn movn_64(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;
    fn movz_64(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;
    fn movk_64(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;

    fn sbfm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn bfm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn ubfm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn sbfm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn bfm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    fn ubfm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;

    fn extr_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, rm: u32)
        -> Result<(), Self::Error>;
    fn extr_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, rm: u32)
        -> Result<(), Self::Error>;

    fn udiv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn sdiv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn lslv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn lsrv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn asrv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn rorv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    fn crc32b(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn crc32h(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn crc32w(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn crc32cb(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn crc32ch(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn crc32cw(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn crc32x(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn crc32cx(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    
    fn subp(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    fn udiv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn sdiv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn irg(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn gmi(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
        
    fn lslv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn lsrv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn asrv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn rorv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    fn pacga(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    fn subps(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
}

#[inline(always)]
fn data_processing_immediate<U: ArmV8aA64User>(user: &mut U, word: u32) 
    -> Result<(), ErrorDissArmV8aA64<U::Error>> {
    match word.extract_bits::<23,25>() {
        // ADR or ADRP
        0b000| 0b001 => {
            // 20 bits immediate
            let imm = (
                    word.extract_bits::<5, 22>() << 2
                ) | word.extract_bits::<29, 30>();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();

            if word.extract_bit::<31>() == 0 {
                user.adr(rd, imm)
            } else {
                user.adrp(rd, imm)
            }.map_err(ErrorDissArmV8aA64::UserError)
        },
        // Add/subtract immediate
        0b010 => {
            let imm = word.extract_bits::<10, 21>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();

            match word.extract_bits::<29, 31>() {
                0b000 => {
                    user.add_imm_32(rd, rn, imm)
                },
                0b001 => {
                    user.adds_imm_32(rd, rn, imm)
                },
                0b010 => {
                    user.sub_imm_32(rd, rn, imm)
                },
                0b011 => {
                    user.subs_imm_32(rd, rn, imm)
                },
                0b100 => {
                    user.add_imm_64(rd, rn, imm)
                },
                0b101 => {
                    user.adds_imm_64(rd, rn, imm)
                },
                0b110 => {
                    user.sub_imm_64(rd, rn, imm)
                },
                0b111 => {
                    user.subs_imm_64(rd, rn, imm)
                },
                _ => {unreachable!();},
            }.map_err(ErrorDissArmV8aA64::UserError)
        },
        // Add/subtract immediate, with tags
        0b011 => {
            unimplemented!("TODO!: figure out ADDG and SUBG");
        },
        // Logical (immediate)
        0b100 => {
            let immr = word.extract_bits::<15, 21>();
            let imms = word.extract_bits::<10, 15>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();

            // instructions with the N value set to 1 are UNALLOCATED
            if word.extract_bit::<22>() != 0 {
                return Err(
                    ErrorDissArmV8aA64::UnallocatedInstruction(word)
                );
            }

            match word.extract_bits::<29, 31>() {
                0b000 => {
                    user.and_imm_32(rd, rn, imms, immr)
                },
                0b001 => {
                    user.orr_imm_32(rd, rn, imms, immr)
                },
                0b010 => {
                    user.and_imm_32(rd, rn, imms, immr)
                },
                0b011 => {
                    user.and_imm_32(rd, rn, imms, immr)
                },
                0b100 => {
                    user.and_imm_64(rd, rn, imms, immr)
                },
                0b101 => {
                    user.orr_imm_64(rd, rn, imms, immr)
                },
                0b110 => {
                    user.and_imm_64(rd, rn, imms, immr)
                },
                0b111 => {
                    user.and_imm_64(rd, rn, imms, immr)
                },
                _ => {unreachable!()},
            }.map_err(ErrorDissArmV8aA64::UserError)
        },
        // Move wide (immediate)
        0b101 => {
            let hw = word.extract_bits::<21, 22>();
            let imm16 = word.extract_bits::<5, 20>();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();

            match word.extract_bits::<29, 31>() {
                0b000 => {
                    user.movn_32(rd, imm16, hw)
                },
                0b010 => {
                    user.movz_32(rd, imm16, hw)
                },
                0b011 => {
                    user.movk_32(rd, imm16, hw)
                },
                0b100 => {
                    user.movn_64(rd, imm16, hw)
                },
                0b110 => {
                    user.movz_64(rd, imm16, hw)
                },
                0b111 => {
                    user.movk_64(rd, imm16, hw)
                },
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                },

            }.map_err(ErrorDissArmV8aA64::UserError)
        },
        // Bitfield
        0b110 => {
            let immr = word.extract_bits::<16, 21>();
            let imms = word.extract_bits::<10, 15>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();
            // build a single opcode so the match is cleaner
            // (sf, opc[2], opc[1], N)
            let opcode = (word.extract_bits::<29, 31>() << 1)
                | word.extract_bit::<22>();
            match opcode {
                0b0000 => {
                    user.sbfm_32(rd, rn, imms, immr)
                },
                0b0010 => {
                    user.bfm_32(rd, rn, imms, immr)
                },
                0b0100 => {
                    user.ubfm_32(rd, rn, imms, immr)
                },
                0b1001 => {
                    user.sbfm_64(rd, rn, imms, immr)
                },
                0b1011 => {
                    user.bfm_64(rd, rn, imms, immr)
                },
                0b1101 => {
                    user.ubfm_64(rd, rn, imms, immr)
                },
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                },
            }.map_err(ErrorDissArmV8aA64::UserError)
        },
        // Extract
        0b111 => {
            let rm = word.extract_bits::<16, 20>();
            let imms = word.extract_bits::<10, 15>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();
            // build a single opcode so that the match is cleaner
            // (sf, opc[2], opc[1], N, o0)
            let opcode = word.extract_bits::<21, 22>() |
                (word.extract_bits::<29, 31>() << 2);

            match opcode {
                0b00000 => {
                    user.extr_32(rd, rn, imms, rm)
                },
                0b10010 => {
                    user.extr_64(rd, rn, imms, rm)
                },
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                },
            }.map_err(ErrorDissArmV8aA64::UserError)
        },
        _ => {unreachable!();},
    }
}

#[inline(always)]
fn data_processing_register<U: ArmV8aA64User>(user: &mut U, word: u32) 
    -> Result<(), ErrorDissArmV8aA64<U::Error>> {
    match (word.extract_bit::<28>(), word.extract_bits::<21, 24>()) {
        // Data-processing (2 source)
        (1, 0b0110) => {
            if word.extract_bit::<30>() == 0 {
                // Data-processing (2 source)
                let rm: RegA64 = word.extract_bits::<16, 20>().into();
                let opcode = word.extract_bits::<10, 15>();
                let rn: RegA64 = word.extract_bits::<5, 9>().into();
                let rd: RegA64 = word.extract_bits::<0, 4>().into();
                // combined opcode for easier match
                // (sd, opcode)
                let op = (word.extract_bit::<31>() << 6) | opcode;
                match op {
                    0b0000010 => {
                        user.udiv_32(rd, rn, rm)
                    },
                    0b0000011 => {
                        user.sdiv_32(rd, rn, rm)
                    },
                    0b0001000 => {
                        user.lslv_32(rd, rn, rm)
                    },
                    0b0001001 => {
                        user.lsrv_32(rd, rn, rm)
                    },
                    0b0001010 => {
                        user.asrv_32(rd, rn, rm)
                    },
                    0b0001011 => {
                        user.rorv_32(rd, rn, rm)
                    },
                    0b0010000 => {
                        user.crc32b(rd, rn, rm)
                    },
                    0b0010001 => {
                        user.crc32h(rd, rn, rm)
                    },
                    0b0010010 => {
                        user.crc32w(rd, rn, rm)
                    },
                    0b0010100 => {
                        user.crc32cb(rd, rn, rm)
                    },
                    0b0010101 => {
                        user.crc32ch(rd, rn, rm)
                    },
                    0b0010110 => {
                        user.crc32cw(rd, rn, rm)   
                    },
                    0b1000000 => {
                        user.subp(rd, rn, rm)
                    },
                    0b1000010 => {
                        user.udiv_64(rd, rn, rm)
                    },
                    0b1000011 => {
                        user.sdiv_64(rd, rn, rm)
                    },
                    0b1000100 => {
                        user.irg(rd, rn, rm)
                    },
                    0b1000101 => {
                        user.gmi(rd, rn, rm)
                    },
                    0b1001000 => {
                        user.lslv_64(rd, rn, rm)
                    },
                    0b1001001 => {
                        user.lsrv_64(rd, rn, rm)
                    },
                    0b1001010 => {
                        user.asrv_64(rd, rn, rm)
                    },
                    0b1001011 => {
                        user.rorv_64(rd, rn, rm)
                    },
                    0b1001100 => {
                        user.pacga(rd, rn, rm)
                    },
                    0b1010011 => {
                        user.crc32x(rd, rn, rm)
                    },
                    0b1010111 => {
                        user.crc32cx(rd, rn, rm)
                    },
                    0b1000000 => {
                        if word.extract_bit::<29>() != 0 {
                            user.subps(rd, rn, rm)
                        } else {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                    },
                    _ => {
                        return Err(
                            ErrorDissArmV8aA64::UnallocatedInstruction(word)
                        );
                    },
                }
            } else {
                // Data-processing (1 source)
            }
        },
        // Logical (shifted register)
        (0, 0b0000 | 0b0001 | 0b0010 | 0b0011 
            | 0b0100 | 0b0101 | 0b0110 | 0b0111) => {
                
        },
        // Add/subtract (shifted register)
        (0, 0b1000 | 0b1010 | 0b1100 | 0b1110) => {

        },
        // Add/subtract (extended register)
        (0, 0b1001 | 0b1011 | 0b1101 | 0b1111) => {
            
        },
        (1, 0b0000) => {
            match word.extract_bits::<10, 15>() {
                // Add/subtract (with carry)
                0b000000 => {

                },
                // Rotate right into flags
                0b000001 | 0b100001 => {

                },
                // Evaluate into flags
                0b000010 | 0b010010 | 0b100010 | 0b110010 => {

                },
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                },
            }
        },
        (1, 0b0010) => {
            if word.extract_bit::<11>() == 0 { 
                // Conditional compare (register)

            } else {
                // Conditional compare (immediate)

            }
        },
        // Conditional select
        (1, 0b0100) => {

        },
        // Data-processing (3 source)
        (1, 0b1000 | 0b1001 | 0b1010 | 0b1011 
            | 0b1100 | 0b1101 | 0b1110 | 0b1111) => {
            
        },
        _ => {unreachable!();}
    }
}

// <https://developer.arm.com/documentation/ddi0602/2021-12/Index-by-Encoding>
pub fn disassemble_armv8a_a64<U: ArmV8aA64User>(user: &mut U, word: u32) 
    -> Result<(), ErrorDissArmV8aA64<U::Error>> {
    match word.extract_bits::<25,28>() {
        0b0000 => {
            if word.extract_bit::<31>() == 0 {
                return Err(ErrorDissArmV8aA64::UnallocatedInstruction(word));
                
            }
            unimplemented!("TODO!: SME");
        }
        // SVE encodings
        0b0010 => {
            unimplemented!("TODO!: SVE")
        }
        // Data Processing -- immediate
        0b1000 | 0b1001 => {
            data_processing_immediate(user, word)
        },
        // Branches, Exception Generating and System instructions
        0b1010 | 0b1011 => {
            unimplemented!("TODO!: Branches, Exception Generating and System instructions")
        },
        // Loads and Stores
        0b0100 | 0b0110 | 0b1100 | 0b1110 => {
            unimplemented!("TODO!: Loads and Stores")
        },
        // Data Processing -- Register
        0b0101 | 0b1101 => {
            data_processing_register(user, word)
        },
        // Data Processing -- Scalar Floating-Point and Advanced SIMD
        0b0111 | 0b1111 => {
            unimplemented!("TODO!: Data Processing -- Scalar Floating-Point and Advanced SIMD")
        },

        x => Err(ErrorDissArmV8aA64::UnallocatedInstruction(x)),
    }
}