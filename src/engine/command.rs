// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// Ported from Command.h and CCommand.h

/// All calculator command IDs, ported directly from the Microsoft Calculator
/// Command.h and CCommand.h. Values match the original C++ enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Command {
    CommandNULL = 0,

    // Sign, clear, backspace
    CommandSIGN = 80,
    CommandCLEAR = 81,
    CommandCENTR = 82,
    CommandBACK = 83,
    CommandPNT = 84,

    // Bitwise / logical operators
    CommandAnd = 86,
    CommandOR = 87,
    CommandXor = 88,
    CommandLSHF = 89,
    CommandRSHF = 90,

    // Arithmetic operators
    CommandDIV = 91,
    CommandMUL = 92,
    CommandADD = 93,
    CommandSUB = 94,
    CommandMOD = 95,
    CommandROOT = 96,
    CommandPWR = 97,

    // Unary operators (between CommandCHOP and CommandEQU)
    CommandCHOP = 98,
    CommandROL = 99,
    CommandROR = 100,
    CommandCOM = 101,

    CommandSIN = 102,
    CommandCOS = 103,
    CommandTAN = 104,
    CommandSINH = 105,
    CommandCOSH = 106,
    CommandTANH = 107,

    CommandLN = 108,
    CommandLOG = 109,
    CommandSQRT = 110,
    CommandSQR = 111,
    CommandCUB = 112,
    CommandFAC = 113,
    CommandREC = 114,
    CommandDMS = 115,
    CommandCUBEROOT = 116,
    CommandPOW10 = 117,
    CommandPERCENT = 118,

    // Display/mode/special
    CommandFE = 119,
    CommandPI = 120,
    CommandEQU = 121,

    // Memory
    CommandMCLEAR = 122,
    CommandRECALL = 123,
    CommandSTORE = 124,
    CommandMPLUS = 125,
    CommandMMINUS = 126,

    CommandEXP = 127,

    // Parentheses
    CommandOPENP = 128,
    CommandCLOSEP = 129,

    // Digits 0-9 and hex A-F
    Command0 = 130,
    Command1 = 131,
    Command2 = 132,
    Command3 = 133,
    Command4 = 134,
    Command5 = 135,
    Command6 = 136,
    Command7 = 137,
    Command8 = 138,
    Command9 = 139,
    CommandA = 140,
    CommandB = 141,
    CommandC = 142,
    CommandD = 143,
    CommandE = 144,
    CommandF = 145,

    CommandINV = 146,
    CommandSET_RESULT = 147,

    // Mode settings
    ModeBasic = 200,
    ModeScientific = 201,

    // Inverse trig
    CommandASIN = 202,
    CommandACOS = 203,
    CommandATAN = 204,
    CommandPOWE = 205,
    CommandASINH = 206,
    CommandACOSH = 207,
    CommandATANH = 208,

    ModeProgrammer = 209,

    // Extended trig
    CommandSEC = 400,
    CommandASEC = 401,
    CommandCSC = 402,
    CommandACSC = 403,
    CommandCOT = 404,
    CommandACOT = 405,
    CommandSECH = 406,
    CommandASECH = 407,
    CommandCSCH = 408,
    CommandACSCH = 409,
    CommandCOTH = 410,
    CommandACOTH = 411,

    CommandPOW2 = 412,
    CommandAbs = 413,
    CommandFloor = 414,
    CommandCeil = 415,
    CommandROLC = 416,
    CommandRORC = 417,

    CommandLogBaseY = 500,
    CommandNand = 501,
    CommandNor = 502,
    CommandRSHFL = 505,

    CommandRand = 600,
    CommandEuler = 601,
}

// IDC_ constants matching CCommand.h defines
pub const IDC_SIGN: u32 = Command::CommandSIGN as u32;
pub const IDC_CLEAR: u32 = Command::CommandCLEAR as u32;
pub const IDC_CENTR: u32 = Command::CommandCENTR as u32;
pub const IDC_BACK: u32 = Command::CommandBACK as u32;
pub const IDC_PNT: u32 = Command::CommandPNT as u32;

pub const IDC_AND: u32 = Command::CommandAnd as u32;
pub const IDC_OR: u32 = Command::CommandOR as u32;
pub const IDC_XOR: u32 = Command::CommandXor as u32;
pub const IDC_LSHF: u32 = Command::CommandLSHF as u32;
pub const IDC_RSHF: u32 = Command::CommandRSHF as u32;
pub const IDC_RSHFL: u32 = Command::CommandRSHFL as u32;

pub const IDC_DIV: u32 = Command::CommandDIV as u32;
pub const IDC_MUL: u32 = Command::CommandMUL as u32;
pub const IDC_ADD: u32 = Command::CommandADD as u32;
pub const IDC_SUB: u32 = Command::CommandSUB as u32;
pub const IDC_MOD: u32 = Command::CommandMOD as u32;
pub const IDC_ROOT: u32 = Command::CommandROOT as u32;
pub const IDC_PWR: u32 = Command::CommandPWR as u32;

pub const IDC_CHOP: u32 = Command::CommandCHOP as u32;
pub const IDC_COM: u32 = Command::CommandCOM as u32;
pub const IDC_SIN: u32 = Command::CommandSIN as u32;
pub const IDC_COS: u32 = Command::CommandCOS as u32;
pub const IDC_TAN: u32 = Command::CommandTAN as u32;
pub const IDC_SINH: u32 = Command::CommandSINH as u32;
pub const IDC_COSH: u32 = Command::CommandCOSH as u32;
pub const IDC_TANH: u32 = Command::CommandTANH as u32;
pub const IDC_LN: u32 = Command::CommandLN as u32;
pub const IDC_LOG: u32 = Command::CommandLOG as u32;
pub const IDC_SQRT: u32 = Command::CommandSQRT as u32;
pub const IDC_SQR: u32 = Command::CommandSQR as u32;
pub const IDC_CUB: u32 = Command::CommandCUB as u32;
pub const IDC_FAC: u32 = Command::CommandFAC as u32;
pub const IDC_REC: u32 = Command::CommandREC as u32;
pub const IDC_DMS: u32 = Command::CommandDMS as u32;
pub const IDC_CUBEROOT: u32 = Command::CommandCUBEROOT as u32;
pub const IDC_POW10: u32 = Command::CommandPOW10 as u32;
pub const IDC_PERCENT: u32 = Command::CommandPERCENT as u32;

pub const IDC_FE: u32 = Command::CommandFE as u32;
pub const IDC_PI: u32 = Command::CommandPI as u32;
pub const IDC_EQU: u32 = Command::CommandEQU as u32;

pub const IDC_MCLEAR: u32 = Command::CommandMCLEAR as u32;
pub const IDC_RECALL: u32 = Command::CommandRECALL as u32;
pub const IDC_STORE: u32 = Command::CommandSTORE as u32;
pub const IDC_MPLUS: u32 = Command::CommandMPLUS as u32;
pub const IDC_MMINUS: u32 = Command::CommandMMINUS as u32;

pub const IDC_EXP: u32 = Command::CommandEXP as u32;
pub const IDC_OPENP: u32 = Command::CommandOPENP as u32;
pub const IDC_CLOSEP: u32 = Command::CommandCLOSEP as u32;

pub const IDC_0: u32 = Command::Command0 as u32;
pub const IDC_INV: u32 = Command::CommandINV as u32;
pub const IDC_SET_RESULT: u32 = Command::CommandSET_RESULT as u32;

pub const IDC_DEGREES: u32 = 324; // from CCommand.h
pub const IDC_SEC: u32 = Command::CommandSEC as u32;
pub const IDC_CSC: u32 = Command::CommandCSC as u32;
pub const IDC_COT: u32 = Command::CommandCOT as u32;
pub const IDC_SECH: u32 = Command::CommandSECH as u32;
pub const IDC_CSCH: u32 = Command::CommandCSCH as u32;
pub const IDC_COTH: u32 = Command::CommandCOTH as u32;
pub const IDC_POW2: u32 = Command::CommandPOW2 as u32;
pub const IDC_ABS: u32 = Command::CommandAbs as u32;
pub const IDC_FLOOR: u32 = Command::CommandFloor as u32;
pub const IDC_CEIL: u32 = Command::CommandCeil as u32;
pub const IDC_RAND: u32 = Command::CommandRand as u32;
pub const IDC_EULER: u32 = Command::CommandEuler as u32;
pub const IDC_LOGBASEY: u32 = Command::CommandLogBaseY as u32;
pub const IDC_NAND: u32 = Command::CommandNand as u32;
pub const IDC_NOR: u32 = Command::CommandNor as u32;

pub const IDC_ROL: u32 = Command::CommandROL as u32;
pub const IDC_ROR: u32 = Command::CommandROR as u32;
pub const IDC_ROLC: u32 = Command::CommandROLC as u32;
pub const IDC_RORC: u32 = Command::CommandRORC as u32;

// First control ID for relative indexing
pub const IDC_FIRSTCONTROL: u32 = IDC_SIGN;

/// Maximum depth for parenthesis/precedence stacks (from History.h)
pub const MAXPRECDEPTH: usize = 25;

// Utility functions ported from CalcUtils.h/CalcUtils.cpp

pub fn is_op_in_range(op: u32, lo: u32, hi: u32) -> bool {
    op >= lo && op <= hi
}

pub fn is_bin_op_code(op: u32) -> bool {
    is_op_in_range(op, IDC_AND, IDC_PWR)
        || op == IDC_LOGBASEY
        || op == IDC_NAND
        || op == IDC_NOR
        || op == IDC_RSHFL
}

pub fn is_unary_op_code(op: u32) -> bool {
    is_op_in_range(op, IDC_CHOP, IDC_PERCENT)
        || is_op_in_range(op, IDC_SEC, IDC_RORC)
        || op == IDC_ABS
        || op == IDC_FLOOR
        || op == IDC_CEIL
        || op == IDC_POW2
        || op == IDC_DEGREES
}

pub fn is_digit_op_code(op: u32) -> bool {
    is_op_in_range(op, IDC_0, IDC_0 + 15) // 0-9 and A-F
}

pub fn is_gui_setting_op_code(op: u32) -> bool {
    op == IDC_INV || op == IDC_FE || op == IDC_EXP || op == IDC_BACK
}
