
/// 64-bit RISC-V registers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Register {
    Zero = 0,
    /// Return Address, Caller Saved
    Ra,
    /// Stack Pointer, Callee Saved
    Sp,
    /// Global Pointer
    Gp,
    /// Thread Pointer
    Tp,
    /// Temp / alternate link register, Caller Saved
    T0,
    /// Temp, Caller Saved
    T1,
    /// Temp, Caller Saved
    T2,
    /// Saved registers / Frame Pointer, Callee Saved
    S0,
    /// Saved registers, Callee Saved
    S1,
    /// Function Arguments / return values, Caller Saved
    A0,
    /// Function Arguments / return values, Caller Saved
    A1,
    /// Function Arguments, Caller Saved
    A2,
    /// Function Arguments, Caller Saved
    A3,
    /// Function Arguments, Caller Saved
    A4,
    /// Function Arguments, Caller Saved
    A5,
    /// Function Arguments, Caller Saved
    A6,
    /// Function Arguments, Caller Saved
    A7,
    /// Saved register, Callee Saved
    S2,
    /// Saved registers, Callee Saved
    S3,
    /// Saved registers, Callee Saved
    S4,
    /// Saved registers, Callee Saved
    S5,
    /// Saved registers, Callee Saved
    S6,
    /// Saved registers, Callee Saved
    S7,
    /// Saved registers, Callee Saved
    S8,
    /// Saved registers, Callee Saved
    S9,
    /// Saved registers, Callee Saved
    S10,
    /// Saved registers, Callee Saved
    S11,
    /// Temp, Caller Saved
    T3,
    /// Temp, Caller Saved
    T4,
    /// Temp, Caller Saved
    T5,
    /// Temp, Caller Saved
    T6,
}

impl Register {
    pub(crate) fn into_prime(&self) -> u16 {
        match self {
            Register::S0 => 0b000,
            Register::S1 => 0b001,
            Register::A0 => 0b010,
            Register::A1 => 0b011,
            Register::A2 => 0b100,
            Register::A3 => 0b101,
            Register::A4 => 0b110,
            Register::A5 => 0b111,
            _ => panic!("Unsupported prime reg")
        }
    } 

    pub(crate) fn from_prime(val: u16) -> Register {
        match val {
            0b000 => Register::S0,
            0b001 => Register::S1,
            0b010 => Register::A0,
            0b011 => Register::A1,
            0b100 => Register::A2,
            0b101 => Register::A3,
            0b110 => Register::A4,
            0b111 => Register::A5,
            _ => panic!("Invalid prime register {}", val),
        }
    } 
}

impl From<u32> for Register {
    fn from(val: u32) -> Self {
        // TODO!: do it properly
        assert!(val < 33);
        unsafe {
            core::ptr::read_unaligned(&(val as usize) as
                                      *const usize as *const Register)
        }
    }
}

impl From<Register> for u32 {
    fn from(value: Register) -> Self {
        value as u32
    }
}

/// 64-bit RISC-V float registers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum FloatRegister {
    /// FP temp, Caller Saved
    FT0,
    /// FP temp, Caller Saved
    FT1,
    /// FP temp, Caller Saved
    FT2,
    /// FP temp, Caller Saved
    FT3, 
    /// FP temp, Caller Saved
    FT4,
    /// FP temp, Caller Saved
    FT5, 
    /// FP temp, Caller Saved
    FT6, 
    /// FP temp, Caller Saved
    FT7, 
    /// FP saved registers, Callee Saved
    FS0,
    /// FP saved registers, Callee Saved
    FS1,
    /// FP Arguments / return values, Caller Saved
    FA0,
    /// FP Arguments / return values, Caller Saved
    FA1,
    /// FP Arguments, Caller Saved
    FA2,
    /// FP Arguments, Caller Saved
    FA3,
    /// FP Arguments, Caller Saved
    FA4,
    /// FP Arguments, Caller Saved
    FA5,
    /// FP Arguments, Caller Saved
    FA6,
    /// FP Arguments, Caller Saved
    FA7,
    /// FP saved registers, Callee Saved
    FS2,
    /// FP saved registers, Callee Saved
    FS3,
    /// FP saved registers, Callee Saved
    FS4,
    /// FP saved registers, Callee Saved
    FS5,
    /// FP saved registers, Callee Saved
    FS6,
    /// FP saved registers, Callee Saved
    FS7,
    /// FP saved registers, Callee Saved
    FS8,
    /// FP saved registers, Callee Saved
    FS9,
    /// FP saved registers, Callee Saved
    FS10,
    /// FP saved registers, Callee Saved
    FS11,
    /// FP temp, Caller Saved
    FT8,
    /// FP temp, Caller Saved
    FT9,
    /// FP temp, Caller Saved
    FT10,
    /// FP temp, Caller Saved
    FT11,
    /// Float Control Status Register
    FCSR,
}

impl FloatRegister {
    pub(crate) fn from_prime(val: u16) -> FloatRegister {
        match val {
            0b000 => FloatRegister::FS0,
            0b001 => FloatRegister::FS1,
            0b010 => FloatRegister::FA0,
            0b011 => FloatRegister::FA1,
            0b100 => FloatRegister::FA2,
            0b101 => FloatRegister::FA3,
            0b110 => FloatRegister::FA4,
            0b111 => FloatRegister::FA5,
            _ => panic!("Invalid prime float register"),
        }
    } 
}

impl From<u32> for FloatRegister {
    fn from(val: u32) -> Self {
        // TODO!: do it properly
        assert!(val < 33);
        unsafe {
            core::ptr::read_unaligned(&(val as usize) as
                                      *const usize as *const FloatRegister)
        }
    }
}

/// 64-bit RISC-V float rounding modes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatRoundingMode {
    /// Round to Nearest, ties to Even
    RNE,
    /// Round towards Zero
    RTZ,
    /// Round Down (torwards -inf)
    RDN,
    /// Round Up (towards +inf)
    RUP,
    /// Round to nearest, ties to Max Magnitude
    RMM,
    /// In instruction's `rm` field, selects dynamic rounding mode;
    /// In Rounding Mode register, reserved
    DYN,
}

impl From<u32> for FloatRoundingMode {
    fn from(val: u32) -> Self {
        match val {
            0b000 => FloatRoundingMode::RNE,
            0b001 => FloatRoundingMode::RTZ,
            0b010 => FloatRoundingMode::RDN,
            0b011 => FloatRoundingMode::RUP,
            0b100 => FloatRoundingMode::RMM,
            0b111 => FloatRoundingMode::DYN,
            _ => unreachable!("Invalid rounding mode"),
        }
    }
}

