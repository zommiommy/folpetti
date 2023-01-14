use super::*;
use traits::*;

/// Disassemble a u32 instruction and call the visitor `User` with the relative
/// Instruction
pub fn diss_riscv64gc<T, User: RV64GCUser<T>>(user: &mut User, inst: u32) 
    -> Result<T, User::Error> {

    // decode the instruction length prefix
    match inst & 0b11 {
        // 4 bytes instruction
        0b11 => diss_riscv64gc_4b_inst(user, inst),
        // 2 bytes instruction (Compact) quadrant 0
        0b00 => diss_riscv64gc_2b_q0_inst(user, inst as u16),
        // 2 bytes instruction (Compact) quadrant 1
        0b01 => diss_riscv64gc_2b_q1_inst(user, inst as u16),
        // 2 bytes instruction (Compact) quadrant 2
        0b10 => diss_riscv64gc_2b_q2_inst(user, inst as u16),
        _ => unreachable!(),
    }
}


fn diss_riscv64gc_2b_q0_inst<T, User: RV64GCUser<T>>(user: &mut User, inst: u16) 
    -> Result<T, User::Error> {
    match inst.extract_bitfield(13, 16) {
        0b000 => {
            let CIWtype{
                funct3, imm, rd_prime
            } = CIWtype::from(inst);
            if imm == 0 {
                panic!("Illegal instruction");
            }
            let nzuimm = ((imm & 0b1) << 3) 
                | ((imm & 0b11100) << 6) 
                | ((imm & 0b1100000) >> 1);
            user.c_addi4spn(Register::from_prime(rd_prime), nzuimm)
        }
        0b001 => {
            let CLtype {
                funct3, imm2, rs1_prime, imm1, rd_prime,
            } = CLtype::from(inst);
            user.c_fld(
                Register::from_prime(rd_prime),
                Register::from_prime(rs1_prime),
                compose_imms_53_76(imm1, imm2),
            )
        }
        0b010 => {
            let CLtype {
                funct3, imm2, rs1_prime, imm1, rd_prime,
            } = CLtype::from(inst);
            user.c_lw(
                Register::from_prime(rd_prime),
                Register::from_prime(rs1_prime),
                compose_imms_53_2_or_6(imm1, imm2),
            )
        }
        0b011 => {
            let CLtype {
                funct3, imm2, rs1_prime, imm1, rd_prime,
            } = CLtype::from(inst);
            user.c_ld(
                Register::from_prime(rd_prime),
                FloatRegister::from_prime(rs1_prime),
                compose_imms_53_76(imm1, imm2),
            )
        }
        0b100 => unimplemented!("Reserved Compact Instruction"),
        0b101 => {
            let CStype {
                funct3, imm2, rs1_prime, imm1, rs2_prime,
            } = CStype::from(inst);
            user.c_fsd(
                Register::from_prime(rs1_prime),
                FloatRegister::from_prime(rs2_prime),
                compose_imms_53_76(imm1, imm2),
            )
        }
        0b110 => {
            let CStype {
                funct3, imm2, rs1_prime, imm1, rs2_prime,
            } = CStype::from(inst);
            user.c_sw(
                Register::from_prime(rs1_prime),
                Register::from_prime(rs2_prime),
                compose_imms_53_2_or_6(imm1, imm2),
            )
        }
        0b111 => {
            let CStype {
                funct3, imm2, rs1_prime, imm1, rs2_prime,
            } = CStype::from(inst);
            user.c_sd(
                Register::from_prime(rs1_prime),
                FloatRegister::from_prime(rs2_prime),
                compose_imms_53_2_or_6(imm1, imm2),
            )
        }
        _ => unreachable!(),
    }
}

fn diss_riscv64gc_2b_q1_inst<T, User: RV64GCUser<T>>(user: &mut User, inst: u16) 
    -> Result<T, User::Error> {
    match inst.extract_bitfield(13, 16) {
        0b000 => {
            let CItype{
                funct3,
                imm2,
                rd_rs1,
                imm1,
            } = CItype::from(inst);

            if rd_rs1 == 0 {
                user.c_nop()
            } else {
                let rd = Register::from(rd_rs1 as u32);
                let imm = if imm2 == 0 {
                    imm1 as i8
                } else {
                    -(imm1 as i8)
                };
                user.c_addi(rd, imm as i8)
            }
        }
        0b001 => {
            let CItype{
                funct3,
                imm2,
                rd_rs1,
                imm1,
            } = CItype::from(inst);
            let rd = Register::from(rd_rs1 as u32);
            let imm = if imm2 == 0 {
                imm1 as i8
            } else {
                -(imm1 as i8)
            };
            user.c_addiw(rd, imm)
        }
        0b010 => {
            let CItype{
                funct3,
                imm2,
                rd_rs1,
                imm1,
            } = CItype::from(inst);
            let rd = Register::from(rd_rs1 as u32);
            let imm = if imm2 == 0 {
                imm1 as i8
            } else {
                -(imm1 as i8)
            };
            user.c_li(rd, imm)
        }
        0b011 => {
            let CItype{
                funct3,
                imm2,
                rd_rs1,
                imm1,
            } = CItype::from(inst);
            let rd = Register::from(rd_rs1 as u32);
            let imm = if imm2 == 0 {
                imm1 as i8
            } else {
                -(imm1 as i8)
            };
            match rd_rs1 {
                0 => unimplemented!(),
                2 => {
                    user.c_addi16sp(imm)
                }
                _ => {
                    user.c_lui(rd, imm)
                }
            }
        }
        0b100 => {
            let CBtype{
                funct3,
                offset2,
                rs1_prime,
                offset1,
            } = CBtype::from(inst);
            let rd = Register::from_prime(rs1_prime);
            match offset2 & 0b111 {
                0b000 => user.c_srli(rd, offset1 as u8),
                0b100 => user.c_srli(rd, 0b10000 | offset1 as u8),
                0b001 => user.c_srai(rd, offset1 as u8),
                0b101 => user.c_srai(rd, 0b10000 | offset1 as u8),
                0b010 => user.c_andi(rd, offset1 as u8),
                0b110 => user.c_andi(rd, 0b10000 | offset1 as u8),
                0b011 => {
                    let rs2 = Register::from_prime(offset1 & 0b111);
                    match (offset1 >> 4) & 0b11 {
                        0b00 => user.c_sub(rd, rs2),
                        0b01 => user.c_xor(rd, rs2),
                        0b10 => user.c_or(rd, rs2),
                        0b11 => user.c_and(rd, rs2),
                        _ => unreachable!(),
                    }
                },
                0b111 => {
                    let rs2 = Register::from_prime(offset1 & 0b111);
                    match (offset1 >> 4) & 0b11 {
                        0b00 => user.c_subw(rd, rs2),
                        0b01 => user.c_addw(rd, rs2),
                        _ => unimplemented!("Reserved"),
                    }
                },
                _ => todo!(),                
            }
        }
        0b101 => {
            todo!()
        }
        0b110 => {
            let CBtype{
                funct3,
                offset2,
                rs1_prime,
                offset1,
            } = CBtype::from(inst);
            let rs1 = Register::from_prime(rs1_prime);

            let offset = ((offset1 & 0b1) << 5) 
                | ((offset1 & 0b110) >> 1) 
                | ((offset1 & 0b11000) << 2)
                | ((offset2 & 0b11) << 2)
                | ((offset2 & 0b100) << 7);

            user.c_beqz(rs1, offset)
        }
        0b111 => {
            let CBtype{
                funct3,
                offset2,
                rs1_prime,
                offset1,
            } = CBtype::from(inst);
            let rs1 = Register::from_prime(rs1_prime);

            let offset = ((offset1 & 0b1) << 5) 
                | ((offset1 & 0b110) >> 1) 
                | ((offset1 & 0b11000) << 2)
                | ((offset2 & 0b11) << 2)
                | ((offset2 & 0b100) << 7);

            user.c_bnez(rs1, offset)
        }
        _ => unreachable!(),
    }
}

fn diss_riscv64gc_2b_q2_inst<T, User: RV64GCUser<T>>(user: &mut User, inst: u16) 
    -> Result<T, User::Error> {
    match inst.extract_bitfield(13, 16) {
        0b011 => {
            let CRtype{
                funct4,
                rd_rs1,
                rs2,
            } = CRtype::from(inst);
            let rd = Register::from(rd_rs1 as u32);
            let imm = (funct4 & 1) << 5 | (
                (rs2 & 0b00111) << 6
            ) | (
                (rs2 & 0b11000) << 3
            );
            user.c_ldsp(rd, imm as u8)
        }
        0b100 => {
            let CRtype{
                funct4,
                rd_rs1,
                rs2,
            } = CRtype::from(inst);

            if (funct4 & 1) == 0 {
                if rs2 == 0 {
                    let rs1 = Register::from(rd_rs1 as u32);
                    user.c_jr(rs1)
                } else {
                    let rs2 = Register::from(rs2 as u32);
                    let rs1 = Register::from(rd_rs1 as u32);
                    user.c_mv(rs1.into(), rs2)
                }
            } else {
                match (rd_rs1, rs2) {
                    (0, 0) => {
                        user.c_ebreak()
                    }
                    (_, 0) => {
                        let rd = Register::from(rd_rs1 as u32);
                        user.c_jalr(rd)
                    }
                    (_, _) => {
                        let rs2 =  Register::from(rs2 as u32);
                        let rd = Register::from(rd_rs1 as u32);
                        user.c_add(rd, rs2)
                    }
                }
            }
        },
        0b111 => {
            let CSStype{
                funct3,
                imm,
                rs2,
            } = CSStype::from(inst);
            let rs2 = FloatRegister::from(rs2 as u32);

            let uimm = (imm & 0b111) << 6
                | (imm & 0b111000);

            debug_assert!(uimm < 256);
            user.c_sdsp(rs2, uimm as u8)
        }
        _ => todo!(),
    }
}

fn diss_riscv64gc_4b_inst<T, User: RV64GCUser<T>>(user: &mut User, inst: u32) 
    -> Result<T, User::Error> {
    // Extract the opcode from the instruction
    let opcode = inst & 0b1111111;

    match opcode {
        0b0110111 => {
            let Utype{imm, rd} = Utype::from(inst);
            user.lui(rd.into(), imm << 12)
        }
        0b0010111 => {
            let Utype{imm, rd} = Utype::from(inst);
            user.auipc(rd.into(), imm << 12)
        }
        0b1101111 => {
            let Jtype{imm, rd} = Jtype::from(inst);
            user.jal(rd.into(), imm as i64 as u64)
        }
        0b1100111 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);
            match funct3 {
                0b000 => {
                    user.jalr(rd.into(), imm as i64 as u64)
                }
                _ => unimplemented!("Unexpected 0b1100111"),
            }
        }
        0b1100011 => {
            let Btype {
                imm, rs2, rs1, funct3,
            } = Btype::from(inst);
            match funct3 {
                0b000 => user.beq( rs1.into(), rs2.into(), imm as i64 as u64),
                0b001 => user.bne( rs1.into(), rs2.into(), imm as i64 as u64),
                0b100 => user.blt( rs1.into(), rs2.into(), imm as i64 as u64),
                0b101 => user.bge( rs1.into(), rs2.into(), imm as i64 as u64),
                0b110 => user.bltu(rs1.into(), rs2.into(), imm as i64 as u64),
                0b111 => user.bgeu(rs1.into(), rs2.into(), imm as i64 as u64),
                _ => unimplemented!("Unexpected 0b1100011"),
            }
        }
        0b0000111 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            todo!();
            /*
            match funct3 {
                0b010 => user.flw(rd.into(), rs1.into(), imm),
                0b011 => user.fld(rd.into(), rs1.into(), imm),
                _ => unimplemented!("Unexpected 0b0000111"),
            }
            */
        }
        0b0100111 => {
            let Stype {
                imm, rs2, rs1, funct3,
            } = Stype::from(inst);

            todo!();
            /*
            match funct3 {
                0b010 => user.fsw(rs1.into(), rs2.into(), imm),
                0b011 => user.fsd(rs1.into(), rs2.into(), imm),
                _ => unimplemented!("Unexpected 0b0000111"),
            }
            */
        }
        0b1000011 => {
            let R4type {
                funct2, rs3, rs2, rs1, funct3, rd,
            } = R4type::from(inst);
            match funct2 {
                00 => user.fmadd_s(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                01 => user.fmadd_d(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                _ => unimplemented!("Unexpected 0b1000011"),
            }
        }
        0b1000111 => {
            let R4type {
                funct2, rs3, rs2, rs1, funct3, rd,
            } = R4type::from(inst);
            match funct2 {
                00 => user.fmsub_s(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                01 => user.fmsub_d(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                _ => unimplemented!("Unexpected 0b1000111"),
            }
        }
        0b1001011 => {
            let R4type {
                funct2, rs3, rs2, rs1, funct3, rd,
            } = R4type::from(inst);
            match funct2 {
                00 => user.fnmsub_s(
                    rd.into(), rs1.into(), rs2.into(),
                    rs3.into(), funct3.into(),
                ),
                01 => user.fnmsub_d(
                    rd.into(), rs1.into(), rs2.into(),
                    rs3.into(), funct3.into(),
                ),
                _ => unimplemented!("Unexpected 0b1001011"),
            }
        }
        0b1001111 => {
            let R4type {
                funct2, rs3, rs2, rs1, funct3, rd,
            } = R4type::from(inst);
            match funct2 {
                00 => user.fnmadd_s(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                01 => user.fnmadd_d(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                _ => unimplemented!("Unexpected 0b1001111"),
            }
        }
        0b1010011 => {
            let Rtype{
                funct7, rs2, rs1, funct3, rd,
            } = Rtype::from(inst);
            match funct7 {
                _ => todo!(),
                0b0000000 => user.fadd_s(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0000001 => user.fadd_d(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0000100 => user.fsub_s(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0000100 => user.fsub_d(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0001000 => user.fmul_s(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0001001 => user.fmul_d(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0001100 => user.fdiv_s(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0001101 => user.fdiv_d(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0101100 => {
                    assert_eq!(rs2, 0b00000);
                    user.fsqrt_s(
                        rd.into(), rs1.into(), 
                        funct3.into(),
                    )
                },
                0b0101101 => {
                    assert_eq!(rs2, 0b00000);
                    user.fsqrt_d(
                        rd.into(), rs1.into(), 
                        funct3.into(),
                    )
                },
                0b0010000 => {
                    match funct3 {
                        0b000 => user.fsgnj_s(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        0b001 => user.fsgnjn_s(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        0b010 => user.fsgnjx_s(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0010000"),
                    }
                }
                0b0010001 => {
                    match funct3 {
                        0b000 => user.fsgnj_d(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        0b001 => user.fsgnjn_d(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        0b010 => user.fsgnjx_d(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0010001"),
                    }
                }
                0b0010100 => {
                    match funct3 {
                        0b000 => user.fmin_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b001 => user.fmax_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0010100"),
                    }
                }
                0b0010101 => {
                    match funct3 {
                        0b000 => user.fmin_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b001 => user.fmax_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0010100"),
                    }
                }
                0b0100000 => {
                    match rs2 {
                        0b00001 => user.fcvt_s_d(
                            rd.into(), rs1.into()
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0100000"),
                    }
                }
                0b0100001 => {
                    match rs2 {
                        0b00000 => user.fcvt_d_s(
                            rd.into(), rs1.into()
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0100001"),
                    }
                }
                0b1100000 => {
                    match rs2 {
                        0b00000 => user.fcvt_w_s(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00001 => user.fcvt_wu_s(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00010 => user.fcvt_l_s(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00011 => user.fcvt_lu_s(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1100000"),
                    }
                }
                0b1100001 => {
                    match rs2 {
                        0b00000 => user.fcvt_w_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00001 => user.fcvt_wu_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00010 => user.fcvt_l_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00011 => user.fcvt_lu_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1100000"),
                    }
                }
                0b1110000 => {
                    match (rs2, funct3) {
                        (0b00000, 0b000) => user.fmv_x_w(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        (0b00000, 0b001) => user.fclass_s(
                            rd.into(), rs1.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1110000"),
                    }
                }
                0b1110001 => {
                    match (rs2, funct3) {
                        (0b00000, 0b000) => user.fmv_x_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        (0b00000, 0b001) => user.fclass_d(
                            rd.into(), rs1.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1110001"),
                    }
                }
                0b1010000 => {
                    match funct3 {
                        0b010 => user.feq_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b001 => user.flt_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b000 => user.fle_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1010000"),
                    }
                }
                0b1010001 => {
                    match funct3 {
                        0b010 => user.feq_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b001 => user.flt_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b000 => user.fle_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1010000"),
                    }
                }
                0b1101000 => {
                    match rs2 {
                        0b00000 => user.fcvt_s_w(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00001 => user.fcvt_s_wu(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00010 => user.fcvt_s_l(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00011 => user.fcvt_s_lu(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1101000"),
                    }
                }
                0b1101001 => {
                    match rs2 {
                        0b00000 => user.fcvt_d_w(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00001 => user.fcvt_d_wu(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00010 => user.fcvt_d_l(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00011 => user.fcvt_d_lu(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1101000"),
                    }
                }
                0b1111000 => {
                    match (rs2, funct3) {
                        (0b00000, 0b000) => user.fmv_w_x(
                            rd.into(), rs1.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1111000"),
                    }
                }
                0b1111001 => {
                    match (rs2, funct3) {
                        (0b00000, 0b000) => user.fmv_d_x(
                            rd.into(), rs1.into(), funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1111001"),
                    }
                }
            }
        }
        0b0000011 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b000 => user.lb( rd.into(), imm as i64 as u64),
                0b001 => user.lh( rd.into(), imm as i64 as u64),
                0b010 => user.lw( rd.into(), imm as i64 as u64),
                0b011 => user.ld( rd.into(), imm as i64 as u64),
                0b100 => user.lbu(rd.into(), imm as i64 as u64),
                0b101 => user.lhu(rd.into(), imm as i64 as u64),
                0b110 => user.lwu(rd.into(), imm as i64 as u64),
                _ => unimplemented!("Unexpected 0b0000011"),
            }
        }
        0b0100011 => {
            let Stype {
                imm, rs2, rs1, funct3,
            } = Stype::from(inst);

            match funct3 {
                0b000 => user.sb(rs1.into(), rs2.into(), imm as i64 as u64),
                0b001 => user.sh(rs1.into(), rs2.into(), imm as i64 as u64),
                0b010 => user.sw(rs1.into(), rs2.into(), imm as i64 as u64),
                0b011 => user.sd(rs1.into(), rs2.into(), imm as i64 as u64),
                _ => unimplemented!("Unexpected 0b0100011"),
            }
        }
        0b0010011 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b000 => user.addi( rd.into(), rs1.into(), imm),
                0b010 => user.slti( rd.into(), rs1.into(), imm),
                0b011 => {
                    let uimm = inst.extract_bitfield(20, 32);
                    user.sltiu(rd.into(), rs1.into(), uimm)
                },
                0b100 => user.xori( rd.into(), rs1.into(), imm),
                0b110 => user.ori(  rd.into(), rs1.into(), imm),
                0b111 => user.andi( rd.into(), rs1.into(), imm),
                0b001 => {
                    let mode = (imm >> 6) & 0b111111;
                    let shamt = imm & 0b111111;
                    
                    match mode {
                        0b000000 => user.slli(rd.into(), rs1.into(), shamt),
                        _ => unreachable!(),
                    }
                }
                0b101 => {
                    let mode = (imm >> 6) & 0b111111;
                    let shamt = imm & 0b111111;
                    
                    match mode {
                        0b000000 => user.srli(rd.into(), rs1.into(), shamt),
                        0b010000 => user.srai(rd.into(), rs1.into(), shamt),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        0b0110011 => {
            let Rtype{
                funct7, rs2, rs1, funct3, rd,
            } = Rtype::from(inst);

            match (funct7, funct3) {
                (0b0000000, 0b000) => user.add(   rd.into(), rs1.into(), rs2.into()),
                (0b0100000, 0b000) => user.sub(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b001) => user.sll(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b010) => user.slt(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b011) => user.sltu(  rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b100) => user.xor(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b101) => user.srl(   rd.into(), rs1.into(), rs2.into()),
                (0b0100000, 0b101) => user.sra(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b110) => user.or(    rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b111) => user.and(   rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b000) => user.mul(   rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b001) => user.mulh(  rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b010) => user.mulhsu(rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b011) => user.mulhu( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b100) => user.div(   rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b101) => user.divu(  rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b110) => user.rem(   rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b111) => user.remu(  rd.into(), rs1.into(), rs2.into()),
                _ => unreachable!(),
            }
        }
        0b0111011 => {
            let Rtype{
                funct7, rs2, rs1, funct3, rd,
            } = Rtype::from(inst);

            match (funct7, funct3) {
                (0b0000000, 0b000) => user.addw( rd.into(), rs1.into(), rs2.into()),
                (0b0100000, 0b000) => user.subw( rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b001) => user.sllw( rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b101) => user.srlw( rd.into(), rs1.into(), rs2.into()),
                (0b0100000, 0b101) => user.sraw( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b000) => user.mulw( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b100) => user.divw( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b101) => user.divuw(rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b110) => user.remw( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b111) => user.remuw(rd.into(), rs1.into(), rs2.into()),
                _ => unreachable!(),
            }
        }
        0b0001111 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b000 => user.fence(),
                0b001 => user.fence_i(),
                _ => unreachable!(),
            }
        }
        0b1110011 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            todo!();
            /*
            match funct3 {
                0b000 => {
                    if inst == 0b00000000000000000000000001110011 {
                        user.ecall()
                    } else if inst == 0b00000000000100000000000001110011 {
                        user.ebreak()
                    } else {
                        unreachable!();
                    }
                }
                0b001 => user.csrrw( rd.into(), rs1.into(), imm),
                0b010 => user.csrrs( rd.into(), rs1.into(), imm),
                0b011 => user.csrrc( rd.into(), rs1.into(), imm),
                0b101 => user.csrrwi(rd.into(), rs1.into(), imm),
                0b110 => user.csrrsi(rd.into(), rs1.into(), imm),
                0b111 => user.csrrci(rd.into(), rs1.into(), imm),
            }*/

        }
        0b0011011 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b000 => user.addiw(rd.into(), rs1.into(), imm as u32),
                0b001 => {
                    let mode = (imm >> 5) & 0b1111111;
                    let shamt = imm & 0b11111;
                    
                    match mode {
                        0b0000000 => user.slliw(rd.into(), rs1.into(), shamt),
                        _ => unreachable!(),
                    }
                }
                0b101 => {
                    let mode = (imm >> 5) & 0b1111111;
                    let shamt = imm & 0b11111;

                    match mode {
                        0b0000000 => user.srliw(rd.into(), rs1.into(), shamt),
                        0b0100000 => user.sraiw(rd.into(), rs1.into(), shamt),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unimplemented!("Unhandled opcode {:#09b}\n", opcode),
    }
}