#[derive(Debug, Clone, PartialEq)]

pub enum Token {
    Identifier(String),
    Contract,
    Library,
    Using,
    Abstract,
    Emit,
    Call,
    Delegatecall,
    Payable,
    Indexed,
    Modifier,
    Interface,
    Revert,
    Space,
    Event,
    Ether,
    Wei,
    Bytes,
    Bytes1,
    Bytes32,
    Assert,
    Require,
    Storage,
    Error,
    Override,
    Push,
    Pop,
    While,
    Delete,
    Enum,
    Immutable,
    Is,
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
    True,
    False,
}

#[derive(Debug)]
pub struct LibraryImplementation {
    pub library: String,
    pub data_type: String,
    pub is_custom_data_type: bool,
    pub is_array: bool,
}

pub enum TerminationType {
    Semicolon,
    None,
    Braces,
}

#[derive(Debug, Clone)]
pub struct VariantType {
    pub type_: String,
    pub name_: String,
    pub size: Option<String>,
    pub is_array: bool,
}

#[derive(Debug)]
pub struct EventIdentifierVariants {
    pub indexed: bool,
    pub variant: String,
}
#[derive(Debug)]
pub struct EventIdentifier {
    pub identifier: String,
    pub variants: Vec<EventIdentifierVariants>,
}

#[derive(Debug)]
pub struct Argument {
    pub type_: String,
    pub name_: String,
    pub location: Option<Token>,
    pub size: Option<String>,
    pub is_array: bool,
    pub payable_address: bool,
}

#[derive(Debug)]
pub struct ReturnType {
    pub type_: String,
    pub location: Option<Token>,
    pub size: Option<String>,
    pub is_array: bool,
}

#[derive(Debug, Clone, PartialEq)]

pub enum VariableType {
    Variable,
    Struct,
    Contract,
    Enum,
}

#[derive(Debug, Clone)]
pub enum StructType {
    Mapping(MappingIdentifier),
    Variant(VariantType),
}

#[derive(Debug, Clone)]
pub struct StructIdentifier {
    pub identifier: String,
    pub is_storage: bool,
    pub types: Vec<StructType>,
}
#[derive(Debug)]
pub struct EnumIdentifier {
    pub identifier: String,
    pub variants: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum RevertType {
    Default,
    Custom,
}

#[derive(Debug, PartialEq)]
pub struct Revert {
    pub r#type: RevertType,
    pub msg: String,
}

#[derive(Debug, Clone, PartialEq)]

pub struct VariableIdentifier {
    pub data_type: Token,
    pub type_: VariableType,
    pub visibility: Token,
    pub mutability: Token,
    pub name: String,
    pub value: Option<String>,
    pub is_array: bool,
    pub size: Option<String>,
    pub storage_location: Option<Token>,
    pub index: Option<u8>,
}

pub enum InterfaceVariants {
    Enum,
    Struct,
    None,
    Function,
    Error,
    Event,
}
#[derive(Debug)]
pub struct ConstructorIdentifier {
    pub arguments: Vec<Argument>,
    pub arms: Vec<FunctionArm>,
    pub initialization: Vec<ConstructorInheritanceInitialization>,
}

#[derive(Debug)]
pub struct ReceiveIdentifier {
    pub arms: Vec<FunctionArm>,
}

#[derive(Debug)]
pub struct FallbackIdentifier {
    pub payable: bool,
    pub arms: Vec<FunctionArm>,
}

#[derive(Debug)]
pub struct CronIdentifier {
    pub min: u8,
    pub hr: u8,
    pub day: u8,
    pub month: u8,
    pub timezone: u8,
    pub arms: Vec<FunctionArm>,
}

#[derive(Debug, PartialEq)]
pub enum CallIdentifierType {
    // Transfer,
    Call,
    Delegatecall,
    // Send,
}

#[derive(Debug, PartialEq)]
pub struct CallIdentifier {
    pub address: String,
    pub payable: bool,
    pub arguments: Vec<String>,
    pub raw_data: Option<[String; 2]>,
    pub r#type: CallIdentifierType,
}

#[derive(Debug)]
pub enum FunctionsIdentifier {
    FunctionIdentifier(FunctionIdentifier),
    ConstructorIdentifier(ConstructorIdentifier),
    ReceiveIdentifier(ReceiveIdentifier),
    FallbackIdentifier(FallbackIdentifier),
    CronIdentifier(CronIdentifier),
    ModifierIdentifier(ModifierIdentifier),
}

#[derive(Debug)]
pub struct LibraryIdentifier {
    pub identifier: String,
    pub constants: Vec<VariableIdentifier>,
    pub enums: Vec<EnumIdentifier>,
    pub structs: Vec<StructIdentifier>,
    pub custom_errors: Vec<CustomErrorIdentifier>,
    pub events: Vec<EventIdentifier>,
    pub functions: Vec<FunctionsIdentifier>,
}

#[derive(Debug)]
pub enum ContractType {
    Abstract,
    Main,
    None,
}

#[derive(Debug)]
pub struct ContractHeader {
    pub identifier: String,
    pub inheritance: Option<Vec<String>>,
    pub r#type: ContractType,
}

#[derive(Debug)]
pub struct ContractIdentifier {
    pub header: ContractHeader,
    pub implementations: Vec<LibraryImplementation>,
    pub state_variables: Vec<VariableIdentifier>,
    pub mappings: Vec<MappingIdentifier>,
    pub enums: Vec<EnumIdentifier>,
    pub structs: Vec<StructIdentifier>,
    pub custom_errors: Vec<CustomErrorIdentifier>,
    pub events: Vec<EventIdentifier>,
    pub functions: Vec<FunctionsIdentifier>,
}

#[derive(Debug)]
pub enum OpenedBraceType {
    None,
    Struct,
    Library,
    Callback,
    Abstract,
    Modifier,
    Function,
    Receive,
    Fallback,
    Contract,
    Enum,
    Constructor,
    Interface,
    Cron,
}

#[derive(Debug, Clone)]
pub struct LineDescriptions {
    pub text: String,
    pub line: i32,
}

#[derive(Debug, PartialEq)]
pub enum FunctionMutability {
    View,
    Pure,
    Mutable,
}

#[derive(Debug)]
pub struct InterfaceIdentifier {
    pub identifier: String,
    pub inheritance: Option<Vec<String>>,
    pub enums: Vec<EnumIdentifier>,
    pub structs: Vec<StructIdentifier>,
    pub custom_errors: Vec<CustomErrorIdentifier>,
    pub events: Vec<EventIdentifier>,
    pub functions: Vec<FunctionHeader>,
}

#[derive(Debug)]
pub struct FunctionHeader {
    pub name: String,
    pub gasless: bool,
    pub mutability: FunctionMutability,
    pub visibility: Token,
    pub returns: Option<Vec<ReturnType>>,
    pub r#override: bool,
    pub r#virtual: bool,
    pub arguments: Vec<Argument>,
    pub modifiers: Option<Vec<ModifierCall>>,
}

#[derive(Debug)]
pub struct CustomErrorIdentifier {
    pub identifier: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct FunctionIdentifier {
    pub header: FunctionHeader,
    pub arms: Vec<FunctionArm>,
}

#[derive(Debug)]
pub struct ConstructorInheritanceInitialization {
    pub identifier: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct ModifierIdentifier {
    pub name: String,
    pub arguments: Vec<Argument>,
    pub arms: Vec<FunctionArm>,
}

#[derive(Debug, PartialEq)]
pub enum VariableAssignType {
    Expression,
    Struct,
    Enum,
    Mapping,
    Array(Option<String>),
}
#[derive(Debug, PartialEq)]
pub struct VariableAssign {
    pub identifier: String,
    pub value: String,
    pub variants: Option<Vec<String>>,
    pub operation: VariableAssignOperation,
    pub type_: VariableAssignType,
}

#[derive(Debug, PartialEq)]
pub struct MappingAssign {
    pub identifier: String,
    pub value: String,
    pub variants: Vec<String>,
    pub operation: VariableAssignOperation,
    pub type_: VariableAssignType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MappingValue {
    Mapping(Box<Mapping>),
    Raw(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mapping {
    pub key: Option<String>,
    pub value: Option<MappingValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MappingIdentifier {
    pub identifier: String,
    pub map: Mapping,
    pub visibility: Token,
}

#[derive(Debug, PartialEq)]
pub struct Delete {
    pub identifier: String,
    pub type_: VariableAssignType,
    pub variants: Option<Vec<String>>,
    pub data_type: Token,
}

#[derive(Debug, PartialEq)]
pub enum VariableAssignOperation {
    Push,
    Pop,
    Assign,
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub enum FunctionCallType {
    Contract(String),
    Local,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub identifier: String,
    pub variant: Option<String>,
    pub arguments: Vec<String>,
    pub r#type: FunctionCallType,
}

#[derive(Debug, PartialEq)]
pub struct ModifierCall {
    pub identifier: String,
    pub arguments: Option<Vec<String>>,
}
#[derive(Debug, PartialEq)]
pub struct Require {
    pub condition: String,
    pub message: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ConditionalType {
    If,
    ElIf,
    El,
    None,
}

#[derive(Debug, PartialEq)]
pub struct ElIf {
    pub condition: Vec<Token>,
    pub arm: Vec<FunctionArm>,
}
#[derive(Debug, PartialEq)]
pub struct Conditionals {
    pub condition: Vec<Token>,
    pub arm: Vec<FunctionArm>,
    pub elif: Vec<ElIf>,
    pub el: Option<Vec<FunctionArm>>,
}

#[derive(Debug, PartialEq)]
pub struct EventEmitter {
    pub identifier: String,
    pub values: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum FunctionArm {
    VariableIdentifier(VariableIdentifier),
    VariableAssign(VariableAssign),
    EventEmitter(EventEmitter),
    MappingAssign(MappingAssign),
    CallIdentifier(CallIdentifier),
    TuppleAssignment(TuppleAssignment),
    FunctionCall(FunctionCall),
    FunctionExecution,
    Break,
    Continue,
    Require(Require),
    Conditionals(Conditionals),
    Return(Return),
    Delete(Delete),
    Revert(Revert),
    Assert(Assert),
    Loop(Loop),
}

#[derive(Debug, PartialEq)]
pub struct Assert {
    pub assert: String,
}

#[derive(Debug, PartialEq)]
pub struct TuppleAssignment {
    pub variables: Vec<VariableIdentifier>,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Loop {
    pub identifier: Option<String>,
    pub value: Option<String>,
    pub condition: String,
    pub iterator: Option<String>,
    pub arms: Vec<FunctionArm>,
    pub r#type: LoopType,
}

#[derive(Debug, PartialEq)]
pub enum LoopType {
    For,
    While,
}

pub enum FunctionArmType {
    StructAssign,
    VariableAssign,
    Conditional,
    Require,
    None,
}
