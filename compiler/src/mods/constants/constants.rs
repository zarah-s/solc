pub const DATA_TYPES: [&str; 6] = ["bytes", "uint", "int", "address", "string", "bool"];

pub const KEYWORDS: [&str; 65] = [
    "contract",
    "mapping",
    "solidity",
    "payable",
    "using",
    "library",
    "abstract",
    "indexed",
    "call",
    "delegatecall",
    "wei",
    "gwei",
    "days",
    "weeks",
    "years",
    "emit",
    "event",
    "ether",
    "error",
    "push",
    "pop",
    "is",
    "import",
    "from",
    "assert",
    "revert",
    "while",
    "immutable",
    "mutable",
    "constant",
    "fallback",
    "calldata",
    "new",
    "cron",
    "delete",
    "receive",
    "gasless",
    "tx",
    "msg",
    "block",
    "pragma",
    "constructor",
    "enum",
    "address",
    "private",
    "struct",
    "function",
    "public",
    "views",
    "returns",
    "pure",
    "return",
    "external",
    "internal",
    "interface",
    "modifier",
    "memory",
    "if",
    "else",
    "for",
    "upgrdable",
    "constant",
    "immutable",
    "true",
    "false",
];

pub const SYMBOLS: [char; 24] = [
    '+', '-', '/', '*', '(', ')', '[', ']', '{', '}', '>', '<', '.', '=', '!', '%', ';', '\'', '"',
    ',', '|', '&', '~', '^',
];

pub const INTEGER_SIZES: [u16; 32] = [
    8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128, 136, 144, 152, 160, 168,
    176, 184, 192, 200, 208, 216, 224, 232, 240, 248, 256,
];
