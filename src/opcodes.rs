use std::fmt;
use strum_macros::{EnumString, Display};

pub const PUSH_OPS: [Opcode; 32] = [
    Opcode::Push1,
    Opcode::Push2,
    Opcode::Push3,
    Opcode::Push4,
    Opcode::Push5,
    Opcode::Push6,
    Opcode::Push7,
    Opcode::Push8,
    Opcode::Push9,
    Opcode::Push10,
    Opcode::Push11,
    Opcode::Push12,
    Opcode::Push13,
    Opcode::Push14,
    Opcode::Push15,
    Opcode::Push16,
    Opcode::Push17,
    Opcode::Push18,
    Opcode::Push19,
    Opcode::Push20,
    Opcode::Push21,
    Opcode::Push22,
    Opcode::Push23,
    Opcode::Push24,
    Opcode::Push25,
    Opcode::Push26,
    Opcode::Push27,
    Opcode::Push28,
    Opcode::Push29,
    Opcode::Push30,
    Opcode::Push31,
    Opcode::Push32,
];

// Source: https://github.com/huff-language/huff-rs/blob/main/huff_utils/src/evm.rs
/// EVM Opcodes
/// References <https://evm.codes>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Opcode {
    /// Halts execution.
    Stop,
    /// Addition operation
    Add,
    /// Multiplication Operation
    Mul,
    /// Subtraction Operation
    Sub,
    /// Integer Division Operation
    Div,
    /// Signed Integer Division Operation
    Sdiv,
    /// Modulo Remainder Operation
    Mod,
    /// Signed Modulo Remainder Operation
    Smod,
    /// Modulo Addition Operation
    Addmod,
    /// Modulo Multiplication Operation
    Mulmod,
    /// Exponential Operation
    Exp,
    /// Extend Length of Two's Complement Signed Integer
    Signextend,
    /// Less-than Comparison
    Lt,
    /// Greater-than Comparison
    Gt,
    /// Signed Less-than Comparison
    Slt,
    /// Signed Greater-than Comparison
    Sgt,
    /// Equality Comparison
    Eq,
    /// Not Operation
    Iszero,
    /// Bitwise AND Operation
    And,
    /// Bitwise OR Operation
    Or,
    /// Bitwise XOR Operation
    Xor,
    /// Bitwise NOT Operation
    Not,
    /// Retrieve Single Byte from Word
    Byte,
    /// Left Shift Operation
    Shl,
    /// Right Shift Operation
    Shr,
    /// Arithmetic Shift Right Operation
    Sar,
    /// Compute the Keccak-256 hash of a 32-byte word
    Sha3,
    /// Address of currently executing account
    Address,
    /// Balance of a given account
    Balance,
    /// Address of execution origination
    Origin,
    /// Address of the caller
    Caller,
    /// Value of the call
    Callvalue,
    /// Loads Calldata
    Calldataload,
    /// Size of the Calldata
    Calldatasize,
    /// Copies the Calldata to Memory
    Calldatacopy,
    /// Size of the Executing Code
    Codesize,
    /// Copies Executing Code to Memory
    Codecopy,
    /// Current Price of Gas
    Gasprice,
    /// Size of an Account's Code
    Extcodesize,
    /// Copies an Account's Code to Memory
    Extcodecopy,
    /// Size of Output Data from Previous Call
    Returndatasize,
    /// Copies Output Data from Previous Call to Memory
    Returndatacopy,
    /// Hash of a Block from the most recent 256 blocks
    Blockhash,
    /// The Current Blocks Beneficiary Address
    Coinbase,
    /// The Current Blocks Timestamp
    Timestamp,
    /// The Current Blocks Number
    Number,
    /// The Current Blocks Difficulty
    Difficulty,
    /// Pseudorandomness from the Beacon Chain
    Prevrandao,
    /// The Current Blocks Gas Limit
    Gaslimit,
    /// The Chain ID
    Chainid,
    /// Balance of the Currently Executing Account
    Selfbalance,
    /// Base Fee
    Basefee,
    /// Removes an Item from the Stack
    Pop,
    /// Loads a word from Memory
    Mload,
    /// Stores a word in Memory
    Mstore,
    /// Stores a byte in Memory
    Mstore8,
    /// Load a word from Storage
    Sload,
    /// Store a word in Storage
    Sstore,
    /// Alter the Program Counter
    Jump,
    /// Conditionally Alter the Program Counter
    Jumpi,
    /// Value of the Program Counter Before the Current Instruction
    Pc,
    /// Size of Active Memory in Bytes
    Msize,
    /// Amount of available gas including the cost of the current instruction
    Gas,
    /// Marks a valid destination for jumps
    Jumpdest,
    /// Places 1 byte item on top of the stack
    Push1,
    /// Places 2 byte item on top of the stack
    Push2,
    /// Places 3 byte item on top of the stack
    Push3,
    /// Places 4 byte item on top of the stack
    Push4,
    /// Places 5 byte item on top of the stack
    Push5,
    /// Places 6 byte item on top of the stack
    Push6,
    /// Places 7 byte item on top of the stack
    Push7,
    /// Places 8 byte item on top of the stack
    Push8,
    /// Places 9 byte item on top of the stack
    Push9,
    /// Places 10 byte item on top of the stack
    Push10,
    /// Places 11 byte item on top of the stack
    Push11,
    /// Places 12 byte item on top of the stack
    Push12,
    /// Places 13 byte item on top of the stack
    Push13,
    /// Places 14 byte item on top of the stack
    Push14,
    /// Places 15 byte item on top of the stack
    Push15,
    /// Places 16 byte item on top of the stack
    Push16,
    /// Places 17 byte item on top of the stack
    Push17,
    /// Places 18 byte item on top of the stack
    Push18,
    /// Places 19 byte item on top of the stack
    Push19,
    /// Places 20 byte item on top of the stack
    Push20,
    /// Places 21 byte item on top of the stack
    Push21,
    /// Places 22 byte item on top of the stack
    Push22,
    /// Places 23 byte item on top of the stack
    Push23,
    /// Places 24 byte item on top of the stack
    Push24,
    /// Places 25 byte item on top of the stack
    Push25,
    /// Places 26 byte item on top of the stack
    Push26,
    /// Places 27 byte item on top of the stack
    Push27,
    /// Places 28 byte item on top of the stack
    Push28,
    /// Places 29 byte item on top of the stack
    Push29,
    /// Places 30 byte item on top of the stack
    Push30,
    /// Places 31 byte item on top of the stack
    Push31,
    /// Places 32 byte item on top of the stack
    Push32,
    /// Duplicates the first stack item
    Dup1,
    /// Duplicates the 2nd stack item
    Dup2,
    /// Duplicates the 3rd stack item
    Dup3,
    /// Duplicates the 4th stack item
    Dup4,
    /// Duplicates the 5th stack item
    Dup5,
    /// Duplicates the 6th stack item
    Dup6,
    /// Duplicates the 7th stack item
    Dup7,
    /// Duplicates the 8th stack item
    Dup8,
    /// Duplicates the 9th stack item
    Dup9,
    /// Duplicates the 10th stack item
    Dup10,
    /// Duplicates the 11th stack item
    Dup11,
    /// Duplicates the 12th stack item
    Dup12,
    /// Duplicates the 13th stack item
    Dup13,
    /// Duplicates the 14th stack item
    Dup14,
    /// Duplicates the 15th stack item
    Dup15,
    /// Duplicates the 16th stack item
    Dup16,
    /// Exchange the top two stack items
    Swap1,
    /// Exchange the first and third stack items
    Swap2,
    /// Exchange the first and fourth stack items
    Swap3,
    /// Exchange the first and fifth stack items
    Swap4,
    /// Exchange the first and sixth stack items
    Swap5,
    /// Exchange the first and seventh stack items
    Swap6,
    /// Exchange the first and eighth stack items
    Swap7,
    /// Exchange the first and ninth stack items
    Swap8,
    /// Exchange the first and tenth stack items
    Swap9,
    /// Exchange the first and eleventh stack items
    Swap10,
    /// Exchange the first and twelfth stack items
    Swap11,
    /// Exchange the first and thirteenth stack items
    Swap12,
    /// Exchange the first and fourteenth stack items
    Swap13,
    /// Exchange the first and fifteenth stack items
    Swap14,
    /// Exchange the first and sixteenth stack items
    Swap15,
    /// Exchange the first and seventeenth stack items
    Swap16,
    /// Append Log Record with no Topics
    Log0,
    /// Append Log Record with 1 Topic
    Log1,
    /// Append Log Record with 2 Topics
    Log2,
    /// Append Log Record with 3 Topics
    Log3,
    /// Append Log Record with 4 Topics
    Log4,
    /// Create a new account with associated code
    Create,
    /// Message-call into an account
    Call,
    /// Message-call into this account with an alternative accounts code
    Callcode,
    /// Halt execution, returning output data
    Return,
    /// Message-call into this account with an alternative accounts code, persisting the sender and
    /// value
    Delegatecall,
    /// Create a new account with associated code
    Create2,
    /// Static Message-call into an account
    Staticcall,
    /// Halt execution, reverting state changes, but returning data and remaining gas
    Revert,
    /// Invalid Instruction
    Invalid,
    /// Halt Execution and Register Account for later deletion
    Selfdestruct,
    /// Get hash of an account’s code
    Extcodehash,
    /// Non-existent opcode
    InvalidOpcode,
}

// Source: https://github.com/huff-language/huff-rs/blob/main/huff_utils/src/evm.rs
impl Opcode {
    /// Translates a hex string into an Opcode
    pub fn new(string: &str) -> Self {
        let opcode = match string {
            "00" => Opcode::Stop,
            "01" => Opcode::Add,
            "02" => Opcode::Mul,
            "03" => Opcode::Sub,
            "04" => Opcode::Div,
            "05" => Opcode::Sdiv,
            "06" => Opcode::Mod,
            "07" => Opcode::Smod,
            "08" => Opcode::Addmod,
            "09" => Opcode::Mulmod,
            "0a" => Opcode::Exp,
            "0b" => Opcode::Signextend,
            "10" => Opcode::Lt,
            "11" => Opcode::Gt,
            "12" => Opcode::Slt,
            "13" => Opcode::Sgt,
            "14" => Opcode::Eq,
            "15" => Opcode::Iszero,
            "16" => Opcode::And,
            "17" => Opcode::Or,
            "18" => Opcode::Xor,
            "19" => Opcode::Not,
            "1a" => Opcode::Byte,
            "1b" => Opcode::Shl,
            "1c" => Opcode::Shr,
            "1d" => Opcode::Sar,
            "20" => Opcode::Sha3,
            "30" => Opcode::Address,
            "31" => Opcode::Balance,
            "32" => Opcode::Origin,
            "33" => Opcode::Caller,
            "34" => Opcode::Callvalue,
            "35" => Opcode::Calldataload,
            "36" => Opcode::Calldatasize,
            "37" => Opcode::Calldatacopy,
            "38" => Opcode::Codesize,
            "39" => Opcode::Codecopy,
            "3a" => Opcode::Gasprice,
            "3b" => Opcode::Extcodesize,
            "3c" => Opcode::Extcodecopy,
            "3d" => Opcode::Returndatasize,
            "3e" => Opcode::Returndatacopy,
            "3f" => Opcode::Extcodehash,
            "40" => Opcode::Blockhash,
            "41" => Opcode::Coinbase,
            "42" => Opcode::Timestamp,
            "43" => Opcode::Number,
            "44" => Opcode::Prevrandao,
            "45" => Opcode::Gaslimit,
            "46" => Opcode::Chainid,
            "47" => Opcode::Selfbalance,
            "48" => Opcode::Basefee,
            "50" => Opcode::Pop,
            "51" => Opcode::Mload,
            "52" => Opcode::Mstore,
            "53" => Opcode::Mstore8,
            "54" => Opcode::Sload,
            "55" => Opcode::Sstore,
            "56" => Opcode::Jump,
            "57" => Opcode::Jumpi,
            "58" => Opcode::Pc,
            "59" => Opcode::Msize,
            "5a" => Opcode::Gas,
            "5b" => Opcode::Jumpdest,
            "60" => Opcode::Push1,
            "61" => Opcode::Push2,
            "62" => Opcode::Push3,
            "63" => Opcode::Push4,
            "64" => Opcode::Push5,
            "65" => Opcode::Push6,
            "66" => Opcode::Push7,
            "67" => Opcode::Push8,
            "68" => Opcode::Push9,
            "69" => Opcode::Push10,
            "6a" => Opcode::Push11,
            "6b" => Opcode::Push12,
            "6c" => Opcode::Push13,
            "6d" => Opcode::Push14,
            "6e" => Opcode::Push15,
            "6f" => Opcode::Push16,
            "70" => Opcode::Push17,
            "71" => Opcode::Push18,
            "72" => Opcode::Push19,
            "73" => Opcode::Push20,
            "74" => Opcode::Push21,
            "75" => Opcode::Push22,
            "76" => Opcode::Push23,
            "77" => Opcode::Push24,
            "78" => Opcode::Push25,
            "79" => Opcode::Push26,
            "7a" => Opcode::Push27,
            "7b" => Opcode::Push28,
            "7c" => Opcode::Push29,
            "7d" => Opcode::Push30,
            "7e" => Opcode::Push31,
            "7f" => Opcode::Push32,
            "80" => Opcode::Dup1,
            "81" => Opcode::Dup2,
            "82" => Opcode::Dup3,
            "83" => Opcode::Dup4,
            "84" => Opcode::Dup5,
            "85" => Opcode::Dup6,
            "86" => Opcode::Dup7,
            "87" => Opcode::Dup8,
            "88" => Opcode::Dup9,
            "89" => Opcode::Dup10,
            "8a" => Opcode::Dup11,
            "8b" => Opcode::Dup12,
            "8c" => Opcode::Dup13,
            "8d" => Opcode::Dup14,
            "8e" => Opcode::Dup15,
            "8f" => Opcode::Dup16,
            "90" => Opcode::Swap1,
            "91" => Opcode::Swap2,
            "92" => Opcode::Swap3,
            "93" => Opcode::Swap4,
            "94" => Opcode::Swap5,
            "95" => Opcode::Swap6,
            "96" => Opcode::Swap7,
            "97" => Opcode::Swap8,
            "98" => Opcode::Swap9,
            "99" => Opcode::Swap10,
            "9a" => Opcode::Swap11,
            "9b" => Opcode::Swap12,
            "9c" => Opcode::Swap13,
            "9d" => Opcode::Swap14,
            "9e" => Opcode::Swap15,
            "9f" => Opcode::Swap16,
            "a0" => Opcode::Log0,
            "a1" => Opcode::Log1,
            "a2" => Opcode::Log2,
            "a3" => Opcode::Log3,
            "a4" => Opcode::Log4,
            "f0" => Opcode::Create,
            "f1" => Opcode::Call,
            "f2" => Opcode::Callcode,
            "f3" => Opcode::Return,
            "f4" => Opcode::Delegatecall,
            "f5" => Opcode::Create2,
            "fa" => Opcode::Staticcall,
            "fd" => Opcode::Revert,
            "fe" => Opcode::Invalid,
            "ff" => Opcode::Selfdestruct,
            _ => Opcode::InvalidOpcode,
        };
        opcode
    }

    /// Translates an Opcode into a string
    pub fn string(&self) -> String {
        let opcode_str = match self {
            Opcode::Stop => "00",
            Opcode::Add => "01",
            Opcode::Mul => "02",
            Opcode::Sub => "03",
            Opcode::Div => "04",
            Opcode::Sdiv => "05",
            Opcode::Mod => "06",
            Opcode::Smod => "07",
            Opcode::Addmod => "08",
            Opcode::Mulmod => "09",
            Opcode::Exp => "0a",
            Opcode::Signextend => "0b",
            Opcode::Lt => "10",
            Opcode::Gt => "11",
            Opcode::Slt => "12",
            Opcode::Sgt => "13",
            Opcode::Eq => "14",
            Opcode::Iszero => "15",
            Opcode::And => "16",
            Opcode::Or => "17",
            Opcode::Xor => "18",
            Opcode::Not => "19",
            Opcode::Byte => "1a",
            Opcode::Shl => "1b",
            Opcode::Shr => "1c",
            Opcode::Sar => "1d",
            Opcode::Sha3 => "20",
            Opcode::Address => "30",
            Opcode::Balance => "31",
            Opcode::Origin => "32",
            Opcode::Caller => "33",
            Opcode::Callvalue => "34",
            Opcode::Calldataload => "35",
            Opcode::Calldatasize => "36",
            Opcode::Calldatacopy => "37",
            Opcode::Codesize => "38",
            Opcode::Codecopy => "39",
            Opcode::Gasprice => "3a",
            Opcode::Extcodesize => "3b",
            Opcode::Extcodecopy => "3c",
            Opcode::Returndatasize => "3d",
            Opcode::Returndatacopy => "3e",
            Opcode::Extcodehash => "3f",
            Opcode::Blockhash => "40",
            Opcode::Coinbase => "41",
            Opcode::Timestamp => "42",
            Opcode::Number => "43",
            Opcode::Difficulty => "44",
            Opcode::Prevrandao => "44",
            Opcode::Gaslimit => "45",
            Opcode::Chainid => "46",
            Opcode::Selfbalance => "47",
            Opcode::Basefee => "48",
            Opcode::Pop => "50",
            Opcode::Mload => "51",
            Opcode::Mstore => "52",
            Opcode::Mstore8 => "53",
            Opcode::Sload => "54",
            Opcode::Sstore => "55",
            Opcode::Jump => "56",
            Opcode::Jumpi => "57",
            Opcode::Pc => "58",
            Opcode::Msize => "59",
            Opcode::Gas => "5a",
            Opcode::Jumpdest => "5b",
            Opcode::Push1 => "60",
            Opcode::Push2 => "61",
            Opcode::Push3 => "62",
            Opcode::Push4 => "63",
            Opcode::Push5 => "64",
            Opcode::Push6 => "65",
            Opcode::Push7 => "66",
            Opcode::Push8 => "67",
            Opcode::Push9 => "68",
            Opcode::Push10 => "69",
            Opcode::Push11 => "6a",
            Opcode::Push12 => "6b",
            Opcode::Push13 => "6c",
            Opcode::Push14 => "6d",
            Opcode::Push15 => "6e",
            Opcode::Push16 => "6f",
            Opcode::Push17 => "70",
            Opcode::Push18 => "71",
            Opcode::Push19 => "72",
            Opcode::Push20 => "73",
            Opcode::Push21 => "74",
            Opcode::Push22 => "75",
            Opcode::Push23 => "76",
            Opcode::Push24 => "77",
            Opcode::Push25 => "78",
            Opcode::Push26 => "79",
            Opcode::Push27 => "7a",
            Opcode::Push28 => "7b",
            Opcode::Push29 => "7c",
            Opcode::Push30 => "7d",
            Opcode::Push31 => "7e",
            Opcode::Push32 => "7f",
            Opcode::Dup1 => "80",
            Opcode::Dup2 => "81",
            Opcode::Dup3 => "82",
            Opcode::Dup4 => "83",
            Opcode::Dup5 => "84",
            Opcode::Dup6 => "85",
            Opcode::Dup7 => "86",
            Opcode::Dup8 => "87",
            Opcode::Dup9 => "88",
            Opcode::Dup10 => "89",
            Opcode::Dup11 => "8a",
            Opcode::Dup12 => "8b",
            Opcode::Dup13 => "8c",
            Opcode::Dup14 => "8d",
            Opcode::Dup15 => "8e",
            Opcode::Dup16 => "8f",
            Opcode::Swap1 => "90",
            Opcode::Swap2 => "91",
            Opcode::Swap3 => "92",
            Opcode::Swap4 => "93",
            Opcode::Swap5 => "94",
            Opcode::Swap6 => "95",
            Opcode::Swap7 => "96",
            Opcode::Swap8 => "97",
            Opcode::Swap9 => "98",
            Opcode::Swap10 => "99",
            Opcode::Swap11 => "9a",
            Opcode::Swap12 => "9b",
            Opcode::Swap13 => "9c",
            Opcode::Swap14 => "9d",
            Opcode::Swap15 => "9e",
            Opcode::Swap16 => "9f",
            Opcode::Log0 => "a0",
            Opcode::Log1 => "a1",
            Opcode::Log2 => "a2",
            Opcode::Log3 => "a3",
            Opcode::Log4 => "a4",
            Opcode::Create => "f0",
            Opcode::Call => "f1",
            Opcode::Callcode => "f2",
            Opcode::Return => "f3",
            Opcode::Delegatecall => "f4",
            Opcode::Create2 => "f5",
            Opcode::Staticcall => "fa",
            Opcode::Revert => "fd",
            Opcode::Invalid => "fe",
            Opcode::Selfdestruct => "ff",
            Opcode::InvalidOpcode => "xx",
        };
        opcode_str.to_string()
    }

    /// Translates an Opcode into opcode string
    pub fn op_string(&self) -> String {
        let opcode_str = match self {
            Opcode::Stop => "Stop",
            Opcode::Add => "Add",
            Opcode::Mul => "Mul",
            Opcode::Sub => "Sub",
            Opcode::Div => "Div",
            Opcode::Sdiv => "Sdiv",
            Opcode::Mod => "Mod",
            Opcode::Smod => "Smod",
            Opcode::Addmod => "Addmod",
            Opcode::Mulmod => "Mulmod",
            Opcode::Exp => "Exp",
            Opcode::Signextend => "Signextend",
            Opcode::Lt => "Lt",
            Opcode::Gt => "Gt",
            Opcode::Slt => "Slt",
            Opcode::Sgt => "Sgt",
            Opcode::Eq => "Eq",
            Opcode::Iszero => "Iszero",
            Opcode::And => "And",
            Opcode::Or => "Or",
            Opcode::Xor => "Xor",
            Opcode::Not => "Not",
            Opcode::Byte => "Byte",
            Opcode::Shl => "Shl",
            Opcode::Shr => "Shr",
            Opcode::Sar => "Sar",
            Opcode::Sha3 => "Sha3",
            Opcode::Address => "Address",
            Opcode::Balance => "Balance",
            Opcode::Origin => "Origin",
            Opcode::Caller => "Caller",
            Opcode::Callvalue => "Callvalue",
            Opcode::Calldataload => "Calldataload",
            Opcode::Calldatasize => "Calldatasize",
            Opcode::Calldatacopy => "Calldatacopy",
            Opcode::Codesize => "Codesize",
            Opcode::Codecopy => "Codecopy",
            Opcode::Gasprice => "Gasprice",
            Opcode::Extcodesize => "Extcodesize",
            Opcode::Extcodecopy => "Extcodecopy",
            Opcode::Returndatasize => "Returndatasize",
            Opcode::Returndatacopy => "Returndatacopy",
            Opcode::Extcodehash => "Extcodehash",
            Opcode::Blockhash => "Blockhash",
            Opcode::Coinbase => "Coinbase",
            Opcode::Timestamp => "Timestamp",
            Opcode::Number => "Number",
            Opcode::Difficulty => "Difficulty",
            Opcode::Prevrandao => "Prevrandao",
            Opcode::Gaslimit => "Gaslimit",
            Opcode::Chainid => "Chainid",
            Opcode::Selfbalance => "Selfbalance",
            Opcode::Basefee => "Basefee",
            Opcode::Pop => "Pop",
            Opcode::Mload => "Mload",
            Opcode::Mstore => "Mstore",
            Opcode::Mstore8 => "Mstore8",
            Opcode::Sload => "Sload",
            Opcode::Sstore => "Sstore",
            Opcode::Jump => "Jump",
            Opcode::Jumpi => "Jumpi",
            Opcode::Pc => "Pc",
            Opcode::Msize => "Msize",
            Opcode::Gas => "Gas",
            Opcode::Jumpdest => "Jumpdest",
            Opcode::Push1 => "Push1",
            Opcode::Push2 => "Push2",
            Opcode::Push3 => "Push3",
            Opcode::Push4 => "Push4",
            Opcode::Push5 => "Push5",
            Opcode::Push6 => "Push6",
            Opcode::Push7 => "Push7",
            Opcode::Push8 => "Push8",
            Opcode::Push9 => "Push9",
            Opcode::Push10 => "Push10",
            Opcode::Push11 => "Push11",
            Opcode::Push12 => "Push12",
            Opcode::Push13 => "Push13",
            Opcode::Push14 => "Push14",
            Opcode::Push15 => "Push15",
            Opcode::Push16 => "Push16",
            Opcode::Push17 => "Push17",
            Opcode::Push18 => "Push18",
            Opcode::Push19 => "Push19",
            Opcode::Push20 => "Push20",
            Opcode::Push21 => "Push21",
            Opcode::Push22 => "Push22",
            Opcode::Push23 => "Push23",
            Opcode::Push24 => "Push24",
            Opcode::Push25 => "Push25",
            Opcode::Push26 => "Push26",
            Opcode::Push27 => "Push27",
            Opcode::Push28 => "Push28",
            Opcode::Push29 => "Push29",
            Opcode::Push30 => "Push30",
            Opcode::Push31 => "Push31",
            Opcode::Push32 => "Push32",
            Opcode::Dup1 => "Dup1",
            Opcode::Dup2 => "Dup2",
            Opcode::Dup3 => "Dup3",
            Opcode::Dup4 => "Dup4",
            Opcode::Dup5 => "Dup5",
            Opcode::Dup6 => "Dup6",
            Opcode::Dup7 => "Dup7",
            Opcode::Dup8 => "Dup8",
            Opcode::Dup9 => "Dup9",
            Opcode::Dup10 => "Dup10",
            Opcode::Dup11 => "Dup11",
            Opcode::Dup12 => "Dup12",
            Opcode::Dup13 => "Dup13",
            Opcode::Dup14 => "Dup14",
            Opcode::Dup15 => "Dup15",
            Opcode::Dup16 => "Dup16",
            Opcode::Swap1 => "Swap1",
            Opcode::Swap2 => "Swap2",
            Opcode::Swap3 => "Swap3",
            Opcode::Swap4 => "Swap4",
            Opcode::Swap5 => "Swap5",
            Opcode::Swap6 => "Swap6",
            Opcode::Swap7 => "Swap7",
            Opcode::Swap8 => "Swap8",
            Opcode::Swap9 => "Swap9",
            Opcode::Swap10 => "Swap10",
            Opcode::Swap11 => "Swap11",
            Opcode::Swap12 => "Swap12",
            Opcode::Swap13 => "Swap13",
            Opcode::Swap14 => "Swap14",
            Opcode::Swap15 => "Swap15",
            Opcode::Swap16 => "Swap16",
            Opcode::Log0 => "Log0",
            Opcode::Log1 => "Log1",
            Opcode::Log2 => "Log2",
            Opcode::Log3 => "Log3",
            Opcode::Log4 => "Log4",
            Opcode::Create => "Create",
            Opcode::Call => "Call",
            Opcode::Callcode => "Callcode",
            Opcode::Return => "Return",
            Opcode::Delegatecall => "Delegatecall",
            Opcode::Create2 => "Create2",
            Opcode::Staticcall => "Staticcall",
            Opcode::Revert => "Revert",
            Opcode::Invalid => "Invalid",
            Opcode::Selfdestruct => "Selfdestruct",
            Opcode::InvalidOpcode => "InvalidOpcode",
        };
        opcode_str.to_string()
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let opcode_str = self.string();
        write!(f, "{}", opcode_str)
    }
}

impl From<Opcode> for String {
    fn from(o: Opcode) -> Self {
        o.string()
    }
}