// #[derive(Debug, Clone)]
#[derive(Debug, Clone, PartialEq)]

pub enum Token {
    Identifier(String),
    Contract,
    Require,
    Storage,
    Error,
    Override,
    Push,
    Pop,
    Delete,
    Enum,
    Immutable,
    Mutable,
    Constant,
    Internal,
    External,
    Virtual,
    Calldata,
    New,
    Mapping,
    Msg,
    Constructor,
    Address,
    Private,
    Struct,
    Function,
    Public,
    View,
    Returns,
    Pure,
    Return,
    Memory,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint120,
    Uint256,
    Receive,
    Fallback,
    Payable,
    Cron,
    Gasless,
    Int8,
    Int,
    Int16,
    Int32,
    Int120,
    Int256,
    String,
    Bool,
    If,
    Else,
    For,
    Plus,
    Minus,
    Divide,
    Multiply,
    OpenParenthesis,
    CloseParenthesis,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenBraces,
    CloseBraces,
    GreaterThan,
    LessThan,
    Dot,
    Equals,
    Bang,
    Modulu,
    SemiColon,
    Quotation,
    Coma,
    Pipe,
    Ampersand,
}

#[derive(Debug, Clone)]
pub struct StructTypes {
    pub type_: String,
    pub name_: String,
    pub size: Option<String>,
    pub is_array: bool,
}

#[derive(Debug)]
pub struct Argument {
    pub type_: String,
    pub name_: String,
    pub location: Option<Token>,
    pub size: Option<String>,
    pub is_array: bool,
}

#[derive(Debug)]
pub struct ReturnType {
    pub type_: String,
    pub location: Option<Token>,
    pub size: Option<String>,
    pub is_array: bool,
}

#[derive(Debug, Clone)]

pub enum VariableType {
    Variable,
    Struct,
    Contract,
    Enum,
}

#[derive(Debug, Clone)]
pub struct StructIdentifier {
    pub identifier: String,
    pub types: Vec<StructTypes>,
}
#[derive(Debug)]

pub struct EnumIdentifier {
    pub identifier: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone)]

pub struct VariableIdentifier {
    pub data_type: Token,
    pub type_: VariableType,
    pub visibility: Token,
    pub mutability: Token,
    pub name: String,
    pub value: Option<String>,
    pub is_array: bool,
    pub size: Option<String>,
}

#[derive(Debug)]
pub enum OpenedBraceType {
    None,
    Struct,
    Callback,
    Function,
    Contract,
    Enum,
}

#[derive(Debug, Clone)]
pub struct LineDescriptions {
    pub text: String,
    pub line: i32,
}

#[derive(Debug)]
pub enum FunctionMutability {
    View,
    Pure,
    Mutable,
}

#[derive(Debug)]
pub struct FunctionIdentifier {
    pub name: String,
    pub gasless: bool,
    pub mutability: FunctionMutability,
    pub visibility: Token,
    pub arguments: Vec<Argument>,
    pub returns: Option<Vec<ReturnType>>,
    pub r#override: bool,
    pub r#virtual: bool,
    pub arms: Vec<FunctionArm>,
}

#[derive(Debug)]
pub enum VariableAssignType {
    Expression,
    Struct,
    Enum,
    Array(Option<String>),
}
#[derive(Debug)]
pub struct VariableAssign {
    pub identifier: String,
    pub value: String,
    pub variant: Option<String>,
    pub operation: VariableAssignOperation,
    pub type_: VariableAssignType,
}

#[derive(Debug)]

pub enum MappingValue {
    Mapping(Box<Mapping>),
    Raw(String),
}

#[derive(Debug)]
pub struct Mapping {
    pub key: Option<String>,
    pub value: Option<MappingValue>,
}

#[derive(Debug)]
pub struct Delete {
    pub identifier: String,
    pub type_: VariableAssignType,
    pub variant: Option<String>,
    pub data_type: Token,
}

#[derive(Debug)]
pub enum VariableAssignOperation {
    Push,
    Pop,
    Assign,
}

#[derive(Debug)]
pub struct Return {
    pub value: String,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub identifier: String,
    pub arguments: Vec<String>,
}
#[derive(Debug)]
pub struct Require {
    pub condition: String,
    pub message: Option<String>,
}

#[derive(Debug)]
pub struct ElIf {
    pub condition: Vec<Token>,
    pub arm: Vec<FunctionArm>,
}
#[derive(Debug)]
pub struct Conditionals {
    pub condition: Vec<Token>,
    pub arm: Vec<FunctionArm>,
    pub elif: Option<Vec<ElIf>>,
    pub el: Option<Vec<FunctionArm>>,
}

#[derive(Debug)]
pub enum FunctionArm {
    VariableIdentifier(VariableIdentifier),
    VariableAssign(VariableAssign),
    TuppleAssignment(TuppleAssignment),
    FunctionCall(FunctionCall),
    Require(Require),
    Conditionals(Conditionals),
    Return(Return),
    Delete(Delete),
}

#[derive(Debug)]
pub struct TuppleAssignment {
    pub variables: Vec<VariableIdentifier>,
    pub value: String,
}

pub struct For {
    pub start: String,
    pub condition: String,
    pub increment: String,
    pub arms: Vec<FunctionArm>,
}

pub enum FunctionArmType {
    StructAssign,
    VariableAssign,
    Conditional,
    Require,
    For,
    None,
}
