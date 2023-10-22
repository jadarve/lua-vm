use std::fmt::{Display, Formatter};
use crate::io::TryReadError;

#[repr(u8)]
pub enum OpCode {
    MOVE = 0x00,
    LOADK,
    LOADKX,
    LOADBOOL,
    LOADNIL,
    GETUPVAL,
    GETTABUP,
    GETTABLE,
    SETTABUP,
    SETUPVAL,
    SETTABLE,
    NEWTABLE,
    SELF,
    ADD,
    SUB,
    MUL,
    MOD,
    POW,
    DIV,
    IDIV,
    BAND,
    BOR,
    BXOR,
    SHL,
    SHR,
    UNM,
    BNOT,
    NOT,
    LEN,
    CONCAT,
    JMP,
    EQ,
    LT,
    LE,
    TEST,
    TESTSET,
    CALL,
    TAILCALL,
    RETURN,
    FORLOOP,
    FORPREP,
    TFORCALL,
    TFORLOOP,
    SETLIST,
    CLOSURE,
    VARARG,
    EXTRAARG,
}


impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::MOVE => { f.write_str("MOVE") }
            OpCode::LOADK => { f.write_str("LOADK") }
            OpCode::LOADKX => { f.write_str("LOADKX") }
            OpCode::LOADBOOL => { f.write_str("LOADBOOL") }
            OpCode::LOADNIL => { f.write_str("LOADNIL") }
            OpCode::GETUPVAL => { f.write_str("GETUPVAL") }
            OpCode::GETTABUP => { f.write_str("GETTABUP") }
            OpCode::GETTABLE => { f.write_str("GETTABLE") }
            OpCode::SETTABUP => { f.write_str("SETTABUP") }
            OpCode::SETUPVAL => { f.write_str("SETUPVAL") }
            OpCode::SETTABLE => { f.write_str("SETTABLE") }
            OpCode::NEWTABLE => { f.write_str("NEWTABLE") }
            OpCode::SELF => { f.write_str("SELF") }
            OpCode::ADD => { f.write_str("ADD") }
            OpCode::SUB => { f.write_str("SUB") }
            OpCode::MUL => { f.write_str("MUL") }
            OpCode::MOD => { f.write_str("MOD") }
            OpCode::POW => { f.write_str("POW") }
            OpCode::DIV => { f.write_str("DIV") }
            OpCode::IDIV => { f.write_str("IDIV") }
            OpCode::BAND => { f.write_str("BAND") }
            OpCode::BOR => { f.write_str("BOR") }
            OpCode::BXOR => { f.write_str("BXOR") }
            OpCode::SHL => { f.write_str("SHL") }
            OpCode::SHR => { f.write_str("SHR") }
            OpCode::UNM => { f.write_str("UNM") }
            OpCode::BNOT => { f.write_str("BNOT") }
            OpCode::NOT => { f.write_str("NOT") }
            OpCode::LEN => { f.write_str("LEN") }
            OpCode::CONCAT => { f.write_str("CONCAT") }
            OpCode::JMP => { f.write_str("JMP") }
            OpCode::EQ => { f.write_str("EQ") }
            OpCode::LT => { f.write_str("LT") }
            OpCode::LE => { f.write_str("LE") }
            OpCode::TEST => { f.write_str("TEST") }
            OpCode::TESTSET => { f.write_str("TESTSET") }
            OpCode::CALL => { f.write_str("CALL") }
            OpCode::TAILCALL => { f.write_str("TAILCALL") }
            OpCode::RETURN => { f.write_str("RETURN") }
            OpCode::FORLOOP => { f.write_str("FORLOOP") }
            OpCode::FORPREP => { f.write_str("FORPREP") }
            OpCode::TFORCALL => { f.write_str("TFORCALL") }
            OpCode::TFORLOOP => { f.write_str("TFORLOOP") }
            OpCode::SETLIST => { f.write_str("SETLIST") }
            OpCode::CLOSURE => { f.write_str("CLOSURE") }
            OpCode::VARARG => { f.write_str("VARARG") }
            OpCode::EXTRAARG => { f.write_str("EXTRAARG") }
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == OpCode::MOVE as u8 => Ok(OpCode::MOVE),
            v if v == OpCode::LOADK as u8 => Ok(OpCode::LOADK),
            v if v == OpCode::LOADKX as u8 => Ok(OpCode::LOADKX),
            v if v == OpCode::LOADBOOL as u8 => Ok(OpCode::LOADBOOL),
            v if v == OpCode::LOADNIL as u8 => Ok(OpCode::LOADNIL),
            v if v == OpCode::GETUPVAL as u8 => Ok(OpCode::GETUPVAL),
            v if v == OpCode::GETTABUP as u8 => Ok(OpCode::GETTABUP),
            v if v == OpCode::GETTABLE as u8 => Ok(OpCode::GETTABLE),
            v if v == OpCode::SETTABUP as u8 => Ok(OpCode::SETTABUP),
            v if v == OpCode::SETUPVAL as u8 => Ok(OpCode::SETUPVAL),
            v if v == OpCode::SETTABLE as u8 => Ok(OpCode::SETTABLE),
            v if v == OpCode::NEWTABLE as u8 => Ok(OpCode::NEWTABLE),
            v if v == OpCode::SELF as u8 => Ok(OpCode::SELF),
            v if v == OpCode::ADD as u8 => Ok(OpCode::ADD),
            v if v == OpCode::SUB as u8 => Ok(OpCode::SUB),
            v if v == OpCode::MUL as u8 => Ok(OpCode::MUL),
            v if v == OpCode::MOD as u8 => Ok(OpCode::MOD),
            v if v == OpCode::POW as u8 => Ok(OpCode::POW),
            v if v == OpCode::DIV as u8 => Ok(OpCode::DIV),
            v if v == OpCode::IDIV as u8 => Ok(OpCode::IDIV),
            v if v == OpCode::BAND as u8 => Ok(OpCode::BAND),
            v if v == OpCode::BOR as u8 => Ok(OpCode::BOR),
            v if v == OpCode::BXOR as u8 => Ok(OpCode::BXOR),
            v if v == OpCode::SHL as u8 => Ok(OpCode::SHL),
            v if v == OpCode::SHR as u8 => Ok(OpCode::SHR),
            v if v == OpCode::UNM as u8 => Ok(OpCode::UNM),
            v if v == OpCode::BNOT as u8 => Ok(OpCode::BNOT),
            v if v == OpCode::NOT as u8 => Ok(OpCode::NOT),
            v if v == OpCode::LEN as u8 => Ok(OpCode::LEN),
            v if v == OpCode::CONCAT as u8 => Ok(OpCode::CONCAT),
            v if v == OpCode::JMP as u8 => Ok(OpCode::JMP),
            v if v == OpCode::EQ as u8 => Ok(OpCode::EQ),
            v if v == OpCode::LT as u8 => Ok(OpCode::LT),
            v if v == OpCode::LE as u8 => Ok(OpCode::LE),
            v if v == OpCode::TEST as u8 => Ok(OpCode::TEST),
            v if v == OpCode::TESTSET as u8 => Ok(OpCode::TESTSET),
            v if v == OpCode::CALL as u8 => Ok(OpCode::CALL),
            v if v == OpCode::TAILCALL as u8 => Ok(OpCode::TAILCALL),
            v if v == OpCode::RETURN as u8 => Ok(OpCode::RETURN),
            v if v == OpCode::FORLOOP as u8 => Ok(OpCode::FORLOOP),
            v if v == OpCode::FORPREP as u8 => Ok(OpCode::FORPREP),
            v if v == OpCode::TFORCALL as u8 => Ok(OpCode::TFORCALL),
            v if v == OpCode::TFORLOOP as u8 => Ok(OpCode::TFORLOOP),
            v if v == OpCode::SETLIST as u8 => Ok(OpCode::SETLIST),
            v if v == OpCode::CLOSURE as u8 => Ok(OpCode::CLOSURE),
            v if v == OpCode::VARARG as u8 => Ok(OpCode::VARARG),
            v if v == OpCode::EXTRAARG as u8 => Ok(OpCode::EXTRAARG),
            _ => Err(())
        }
    }
}