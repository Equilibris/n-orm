use super::*;
use crate::*;

pub type KwAs = FIdent<"as">;
pub type KwBreak = FIdent<"break">;
pub type KwConst = FIdent<"const">;
pub type KwContinue = FIdent<"continue">;
pub type KwCrate = FIdent<"crate">;
pub type KwElse = FIdent<"else">;
pub type KwEnum = FIdent<"enum">;
pub type KwExtern = FIdent<"extern">;
pub type KwFalse = FIdent<"false">;
pub type KwFn = FIdent<"fn">;
pub type KwFor = FIdent<"for">;
pub type KwIf = FIdent<"if">;
pub type KwImpl = FIdent<"impl">;
pub type KwIn = FIdent<"in">;
pub type KwLet = FIdent<"let">;
pub type KwLoop = FIdent<"loop">;
pub type KwMatch = FIdent<"match">;
pub type KwMod = FIdent<"mod">;
pub type KwMove = FIdent<"move">;
pub type KwMut = FIdent<"mut">;
pub type KwPub = FIdent<"pub">;
pub type KwRef = FIdent<"ref">;
pub type KwReturn = FIdent<"return">;
pub type KwLowerSelf = FIdent<"self">;
pub type KwUpperSelf = FIdent<"Self">;
pub type KwStatic = FIdent<"static">;
pub type KwStruct = FIdent<"struct">;
pub type KwSuper = FIdent<"super">;
pub type KwTrait = FIdent<"trait">;
pub type KwTrue = FIdent<"true">;
pub type KwType = FIdent<"type">;
pub type KwUnsafe = FIdent<"unsafe">;
pub type KwUse = FIdent<"use">;
pub type KwWhere = FIdent<"where">;
pub type KwWhile = FIdent<"while">;

// 2018
pub type KwAsync = FIdent<"async">;
pub type KwAwait = FIdent<"await">;
pub type KwDyn = FIdent<"dyn">;

pub enum Strict {
    As(KwAs),
    Break(KwBreak),
    Const(KwConst),
    Continue(KwContinue),
    Crate(KwCrate),
    Else(KwElse),
    Enum(KwEnum),
    Extern(KwExtern),
    False(KwFalse),
    Fn(KwFn),
    For(KwFor),
    If(KwIf),
    Impl(KwImpl),
    In(KwIn),
    Let(KwLet),
    Loop(KwLoop),
    Match(KwMatch),
    Mod(KwMod),
    Move(KwMove),
    Mut(KwMut),
    Pub(KwPub),
    Ref(KwRef),
    Return(KwReturn),
    LowerSelf(KwLowerSelf),
    UpperSelf(KwUpperSelf),
    Static(KwStatic),
    Struct(KwStruct),
    Super(KwSuper),
    Trait(KwTrait),
    True(KwTrue),
    Type(KwType),
    Unsafe(KwUnsafe),
    Use(KwUse),
    Where(KwWhere),
    While(KwWhile),
    Async(KwAsync),
    Await(KwAwait),
    Dyn(KwDyn),
}

impl Parsable for Strict {
    // type StateMachine = <MapOut<InnerKws, Keywords> as Parsable>::StateMachine;
    type StateMachine = StrictMachine;
}

#[derive(Default)]
pub struct StrictMachine;

#[derive(Default, thiserror::Error, Debug)]
pub enum KeywordError {
    #[error("Expected keyword but got {}", .0)]
    Val(TokenTree),

    // TODO:
    // #[error("")]
    // Similar()
    #[default]
    #[error("Expected keyword but got termination")]
    Terminate,
}

impl StateMachine for StrictMachine {
    type Output = Strict;
    type Error = KeywordError;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        let v = val.clone();

        match v {
            TokenTree::Ident(v) => {
                let s = v.to_string();

                ControlFlow::Break(Ok((
                    match s.as_str() {
                        "as" => Strict::As(v.into()),
                        "break" => Strict::Break(v.into()),
                        "const" => Strict::Const(v.into()),
                        "continue" => Strict::Continue(v.into()),
                        "crate" => Strict::Crate(v.into()),
                        "else" => Strict::Else(v.into()),
                        "enum" => Strict::Enum(v.into()),
                        "extern" => Strict::Extern(v.into()),
                        "false" => Strict::False(v.into()),
                        "fn" => Strict::Fn(v.into()),
                        "for" => Strict::For(v.into()),
                        "if" => Strict::If(v.into()),
                        "impl" => Strict::Impl(v.into()),
                        "in" => Strict::In(v.into()),
                        "let" => Strict::Let(v.into()),
                        "loop" => Strict::Loop(v.into()),
                        "match" => Strict::Match(v.into()),
                        "mod" => Strict::Mod(v.into()),
                        "move" => Strict::Move(v.into()),
                        "mut" => Strict::Mut(v.into()),
                        "pub" => Strict::Pub(v.into()),
                        "ref" => Strict::Ref(v.into()),
                        "return" => Strict::Return(v.into()),
                        "self" => Strict::LowerSelf(v.into()),
                        "Self" => Strict::UpperSelf(v.into()),
                        "static" => Strict::Static(v.into()),
                        "struct" => Strict::Struct(v.into()),
                        "super" => Strict::Super(v.into()),
                        "trait" => Strict::Trait(v.into()),
                        "true" => Strict::True(v.into()),
                        "type" => Strict::Type(v.into()),
                        "unsafe" => Strict::Unsafe(v.into()),
                        "use" => Strict::Use(v.into()),
                        "where" => Strict::Where(v.into()),
                        "while" => Strict::While(v.into()),
                        "async" => Strict::Async(v.into()),
                        "await" => Strict::Await(v.into()),
                        "dyn" => Strict::Dyn(v.into()),
                        _ => return ControlFlow::Break(Err(Self::Error::Val(TokenTree::Ident(v)))),
                    },
                    0,
                )))
            }
            _ => ControlFlow::Break(Err(Self::Error::Val(v))),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(Default::default())
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}Strict", "  ".repeat(depth));
    }
}

pub type KwAbstract = FIdent<"abstract">;
pub type KwBecome = FIdent<"become">;
pub type KwBox = FIdent<"box">;
pub type KwDo = FIdent<"do">;
pub type KwFinal = FIdent<"final">;
pub type KwMacro = FIdent<"macro">;
pub type KwOverride = FIdent<"override">;
pub type KwPriv = FIdent<"priv">;
pub type KwTypeof = FIdent<"typeof">;
pub type KwUnsized = FIdent<"unsized">;
pub type KwVirtual = FIdent<"virtual">;
pub type KwYield = FIdent<"yield">;

pub enum Reserved {
    Abstract(KwAbstract),
    Become(KwBecome),
    Box(KwBox),
    Do(KwDo),
    Final(KwFinal),
    Macro(KwMacro),
    Override(KwOverride),
    Priv(KwPriv),
    Typeof(KwTypeof),
    Unsized(KwUnsized),
    Virtual(KwVirtual),
    Yield(KwYield),
}

impl Parsable for Reserved {
    // type StateMachine = <MapOut<InnerKws, Keywords> as Parsable>::StateMachine;
    type StateMachine = ReservedMachine;
}

#[derive(Default)]
pub struct ReservedMachine;

impl StateMachine for ReservedMachine {
    type Output = Reserved;
    type Error = KeywordError;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        let v = val.clone();

        match v {
            TokenTree::Ident(v) => {
                let s = v.to_string();

                ControlFlow::Break(Ok((
                    match s.as_str() {
                        "abstract" => Reserved::Abstract(v.into()),
                        "become" => Reserved::Become(v.into()),
                        "box" => Reserved::Box(v.into()),
                        "do" => Reserved::Do(v.into()),
                        "final" => Reserved::Final(v.into()),
                        "macro" => Reserved::Macro(v.into()),
                        "override" => Reserved::Override(v.into()),
                        "priv" => Reserved::Priv(v.into()),
                        "typeof" => Reserved::Typeof(v.into()),
                        "unsized" => Reserved::Unsized(v.into()),
                        "virtual" => Reserved::Virtual(v.into()),
                        "yield" => Reserved::Yield(v.into()),
                        _ => return ControlFlow::Break(Err(Self::Error::Val(TokenTree::Ident(v)))),
                    },
                    0,
                )))
            }
            _ => ControlFlow::Break(Err(Self::Error::Val(v))),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(Default::default())
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}Keyword", "  ".repeat(depth));
    }
}

pub enum Keyword {
    Strict(Strict),
    Reserved(Reserved),
}

impl MappedParse for Keyword {
    type Source = Sum2<Strict, Reserved>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(src: SmOut<<Self as MappedParse>::Source>) -> Result<Self::Output, Self::Error> {
        Ok(match src {
            Sum2::Val0(a) => Self::Strict(a),
            Sum2::Val1(a) => Self::Reserved(a),
        })
    }

    fn map_err(src: SmErr<<Self as MappedParse>::Source>) -> Self::Error {
        src
    }
}

pub type KwUnion = FIdent<"union">;
pub type StaticLifetime = (FJointPunct<'\''>, KwStatic);
pub type UnderLifetime = (FJointPunct<'\''>, Underscore);
pub type DollarCrate = (FPunct<'$'>, FIdent<"crate">);
