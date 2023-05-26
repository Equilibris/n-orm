use super::*;
use crate::*;

pub struct MacroRulesDefinition(pub Ident, pub MacroRulesDef);
impl MappedParse for MacroRulesDefinition {
    type Source = (
        FIdent<"macro_rules">,
        Exclamation,
        Identifier,
        MacroRulesDef,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.2, src.3))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct MacroRulesDef(pub MacroRules);
impl MappedParse for MacroRulesDef {
    type Source = Sum2<(Sum2<Paren<MacroRules>, Bracket<MacroRules>>, Semi), Brace<MacroRules>>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(match src {
            Sum2::Val0((Sum2::Val0(a), _)) => a.0,
            Sum2::Val0((Sum2::Val1(a), _)) => a.0,
            Sum2::Val1(a) => a.0,
        }))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub struct MacroRules(pub Vec<MacroRule>);
impl MappedParse for MacroRules {
    type Source = InterlaceTrail<MacroRule, Semi>;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct MacroRule(MacroMatcher, MacroTranscriber);
impl MappedParse for MacroRule {
    type Source = (MacroMatcher, FatArrow, MacroTranscriber);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self(src.0, src.2))
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

pub type MacroTranscriber = DelimTokenTree;
pub type MacroMatcher = Group<Vec<MacroMatch>>;
#[derive(Debug)]
pub enum MacroMatch {
    Rep {
        content: Vec<MacroMatch>,
        rep_sep: Option<MacroRepSep>,
        rep_op: MacroRepOp,
    },
    Match {
        id: Ident,
        frag: MacroFragSpec,
    },
    Deep(MacroMatcher),
    Token(TokenTree),
}
impl MappedParse for MacroMatch {
    type Source = Sum4<
        (Dollar, Ident, Colon, MacroFragSpec),
        (
            Dollar,
            Paren<Vec<MacroMatch>>,
            Option<MacroRepSep>,
            MacroRepOp,
        ),
        MacroMatcher,
        AndNot<TokenTree, Dollar>,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src {
            Sum4::Val0(a) => Self::Match { id: a.1, frag: a.3 },
            Sum4::Val1(a) => Self::Rep {
                content: a.1 .0,
                rep_sep: a.2,
                rep_op: a.3,
            },
            Sum4::Val2(a) => Self::Deep(a),
            Sum4::Val3(a) => Self::Token(a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub enum MacroFragSpec {
    Block,
    Expr,
    Ident,
    Item,
    Lifetime,
    Literal,
    Meta,
    Pat,
    PatParam,
    Path,
    Stmt,
    Tt,
    Ty,
    Vis,
}
#[derive(Debug, thiserror::Error)]
pub enum MacroFragSpecErr {
    #[error("Expected *, + or ? but got {}", .0)]
    Got(Ident),
    #[error("Expected *, + or ? but err {}", .0)]
    Err(IdentError),
}
impl MappedParse for MacroFragSpec {
    type Source = Ident;

    type Output = Self;
    type Error = MacroFragSpecErr;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        match src.to_string().as_str() {
            "block" => Ok(Self::Block),
            "expr" => Ok(Self::Expr),
            "ident" => Ok(Self::Ident),
            "item" => Ok(Self::Item),
            "lifetime" => Ok(Self::Lifetime),
            "literal" => Ok(Self::Literal),
            "meta" => Ok(Self::Meta),
            "pat" => Ok(Self::Pat),
            "pat_param" => Ok(Self::PatParam),
            "path" => Ok(Self::Path),
            "stmt" => Ok(Self::Stmt),
            "tt" => Ok(Self::Tt),
            "ty" => Ok(Self::Ty),
            "vis" => Ok(Self::Vis),
            _ => Err(MacroFragSpecErr::Got(src)),
        }
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        MacroFragSpecErr::Err(src)
    }
}

#[derive(Debug)]
pub struct MacroRepSep(TokenTree);
#[derive(Debug, thiserror::Error)]
pub enum MacroRepSepErr {
    #[error("Expected MacroRepSep but got delim {}", .0)]
    Got(proc_macro2::Group),
    #[error("Expected *, + or ? but err {}", .0)]
    Err(TokenTreeError),
}
impl MappedParse for MacroRepSep {
    type Source = TokenTree;

    type Output = Self;
    type Error = MacroRepSepErr;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        match src {
            TokenTree::Group(a) => Err(MacroRepSepErr::Got(a)),
            a => Ok(Self(a)),
        }
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        MacroRepSepErr::Err(src)
    }
}

#[derive(Debug)]
pub enum MacroRepOp {
    ZeroOrMore,
    OneOrMore,
    Optional,
}
#[derive(Debug, thiserror::Error)]
pub enum MacroRepOpErr {
    #[error("Expected *, + or ? but got {}", .0)]
    Got(char),
    #[error("Expected *, + or ? but err {}", .0)]
    Err(PunctError),
}
impl MappedParse for MacroRepOp {
    type Source = Punct;

    type Output = Self;
    type Error = MacroRepOpErr;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        match src.as_char() {
            '+' => Ok(MacroRepOp::OneOrMore),
            '*' => Ok(MacroRepOp::ZeroOrMore),
            '?' => Ok(MacroRepOp::Optional),
            a => Err(MacroRepOpErr::Got(a)),
        }
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        MacroRepOpErr::Err(src)
    }
}
