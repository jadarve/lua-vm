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