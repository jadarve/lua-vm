use std::fmt::{Display, Formatter};
// use crate::vm53::instruction::InstructionFormat::{AB, ABC, ABx, AsBx, Ax};
use crate::vm53::OpCode;

pub struct Instruction {
    raw_code: u32,
}

const SIZE_INSTRUCTION: u32 = 32;
const SIZE_OPCODE: u32 = 6;
const SIZE_REGISTER_A: u32 = 8;
const SIZE_REGISTER_B: u32 = 9;
const SIZE_REGISTER_C: u32 = 9;
const SIZE_REGISTER_BX: u32 = SIZE_REGISTER_B + SIZE_REGISTER_C;
const SIZE_REGISTER_AX: u32 = SIZE_REGISTER_A + SIZE_REGISTER_B + SIZE_REGISTER_C;

const SHIFT_REGISTER_A: u32 = SIZE_OPCODE;
const SHIFT_REGISTER_C: u32 = SIZE_OPCODE + SIZE_REGISTER_A;
const SHIFT_REGISTER_B: u32 = SIZE_OPCODE + SIZE_REGISTER_A + SIZE_REGISTER_C;
const SHIFT_REGISTER_BX: u32 = SHIFT_REGISTER_C;
const SHIFT_REGISTER_AX: u32 = SHIFT_REGISTER_A;


const MASK_INSTRUCTION: u32 = 0b00000000_00000000_00000000_00111111;
const MASK_REGISTER_A: u32  = 0b00000000_00000000_00111111_11000000;
const MASK_REGISTER_C: u32  = 0b00000000_01111111_11000000_00000000;
const MASK_REGISTER_B: u32  = 0b11111111_10000000_00000000_00000000;
const MASK_REGISTER_AX: u32 = 0b11111111_11111111_11111111_11000000;
const MASK_REGISTER_BX: u32 = 0b11111111_11111111_11000000_00000000;

#[repr(u32)]
enum InstructionFormat {
    A,
    AB,
    ABC,
    ABx,
    AC,
    AsBx,
    Ax,
}

impl InstructionFormat {
    pub fn from_opcode(opcode: &OpCode) -> Self {
        match opcode {
            OpCode::MOVE => { InstructionFormat::AB }
            OpCode::LOADK => { InstructionFormat::ABx }
            OpCode::LOADKX => { InstructionFormat::A }
            OpCode::LOADBOOL => { InstructionFormat::ABC }
            OpCode::LOADNIL => { InstructionFormat::AB }
            OpCode::GETUPVAL => { InstructionFormat::AB }
            OpCode::GETTABUP => { InstructionFormat::ABC }
            OpCode::GETTABLE => { InstructionFormat::ABC }
            OpCode::SETTABUP => { InstructionFormat::ABC }
            OpCode::SETUPVAL => { InstructionFormat::AB }
            OpCode::SETTABLE => { InstructionFormat::ABC }
            OpCode::NEWTABLE => { InstructionFormat::ABC }
            OpCode::SELF => { InstructionFormat::ABC }
            OpCode::ADD => { InstructionFormat::ABC }
            OpCode::SUB => { InstructionFormat::ABC }
            OpCode::MUL => { InstructionFormat::ABC }
            OpCode::MOD => { InstructionFormat::ABC }
            OpCode::POW => { InstructionFormat::ABC }
            OpCode::DIV => { InstructionFormat::ABC }
            OpCode::IDIV => { InstructionFormat::ABC }
            OpCode::BAND => { InstructionFormat::ABC }
            OpCode::BOR => { InstructionFormat::ABC }
            OpCode::BXOR => { InstructionFormat::ABC }
            OpCode::SHL => { InstructionFormat::ABC }
            OpCode::SHR => { InstructionFormat::ABC }
            OpCode::UNM => { InstructionFormat::AB }
            OpCode::BNOT => { InstructionFormat::AB }
            OpCode::NOT => { InstructionFormat::AB }
            OpCode::LEN => { InstructionFormat::AB }
            OpCode::CONCAT => { InstructionFormat::ABC }
            OpCode::JMP => { InstructionFormat::AsBx }
            OpCode::EQ => { InstructionFormat::ABC }
            OpCode::LT => { InstructionFormat::ABC }
            OpCode::LE => { InstructionFormat::ABC }
            OpCode::TEST => { InstructionFormat::AC }
            OpCode::TESTSET => { InstructionFormat::ABC }
            OpCode::CALL => { InstructionFormat::ABC }
            OpCode::TAILCALL => { InstructionFormat::ABC }
            OpCode::RETURN => { InstructionFormat::AB }
            OpCode::FORLOOP => { InstructionFormat::AsBx }
            OpCode::FORPREP => { InstructionFormat::AsBx }
            OpCode::TFORCALL => { InstructionFormat::AC }
            OpCode::TFORLOOP => { InstructionFormat::AsBx }
            OpCode::SETLIST => { InstructionFormat::ABC }
            OpCode::CLOSURE => { InstructionFormat::ABx }
            OpCode::VARARG => { InstructionFormat::AB }
            OpCode::EXTRAARG => { InstructionFormat::Ax }
        }
    }
}

impl Instruction {
    pub fn new(code: u32) -> Self {
        Instruction { raw_code: code }
    }

    pub fn opcode(&self) -> Result<OpCode, ()> {
        let opcode = self.raw_code & MASK_INSTRUCTION;
        OpCode::try_from(opcode as u8)
    }

    pub fn register_a(&self) -> u32 {
        (self.raw_code & MASK_REGISTER_A) >> SHIFT_REGISTER_A
    }

    pub fn register_ax(&self) -> u32 {
        (self.raw_code & MASK_REGISTER_AX) >> SHIFT_REGISTER_AX
    }

    pub fn register_b(&self) -> u32 {
        (self.raw_code & MASK_REGISTER_B) >> SHIFT_REGISTER_B
    }

    pub fn register_bx(&self) -> u32 {
        (self.raw_code & MASK_REGISTER_BX) >> SHIFT_REGISTER_BX
    }

    pub fn register_c(&self) -> u32 {
        (self.raw_code & MASK_REGISTER_C) >> SHIFT_REGISTER_C
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.opcode() {
            Err(()) => { f.write_str(format!("INVALID").as_str()) }
            Ok(opcode) => {
                match InstructionFormat::from_opcode(&opcode) {
                    InstructionFormat::A => {
                        f.write_str(format!("{} {}", opcode, self.register_a()).as_str())
                    }
                    InstructionFormat::AB => {
                        f.write_str(format!("{} {} {}", opcode, self.register_a(), self.register_b()).as_str())
                    }
                    InstructionFormat::ABC => {
                        f.write_str(format!("{} {} {} {}", opcode, self.register_a(), self.register_b(), self.register_c()).as_str())
                    }
                    InstructionFormat::ABx => {
                        f.write_str(format!("{} {} {}", opcode, self.register_a(), self.register_bx()).as_str())
                    }
                    InstructionFormat::AC => {
                        f.write_str(format!("{} {} {}", opcode, self.register_a(), self.register_c()).as_str())
                    }
                    InstructionFormat::AsBx => {
                        f.write_str(format!("{} {} {}", opcode, self.register_a(), self.register_bx()).as_str())
                    }
                    InstructionFormat::Ax => {
                        f.write_str(format!("{} {}", opcode, self.register_ax()).as_str())
                    }

                }
            }
        }
    }
}