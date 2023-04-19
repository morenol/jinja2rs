use logos::{Lexer, Logos};
use std::borrow::Cow;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Unknown,

    // One-symbol operators
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("%")]
    Percent,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("(")]
    LBracket,
    #[token(")")]
    RBracket,
    #[token("[")]
    LSqBracket,
    #[token("]")]
    RSqBracket,
    #[token("{")]
    LCrlBracket,
    #[token("}")]
    RCrlBracket,
    #[token("=")]
    Assign,
    #[token(".")]
    Point,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("|")]
    Pipe,
    #[token("~")]
    Tilde,

    // General
    #[regex("[A-Za-z_]+[A-Za-z_0-9]*", |lex| Cow::Borrowed(lex.slice())) ]
    Identifier(Cow<'a, str>),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    IntegerNum(i64),
    #[regex("[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    FloatNum(f64),
    #[regex(r#""(?:[^"]|\\")*""#, |lex| Cow::Borrowed(&lex.slice()[1..lex.slice().len()-1]))]
    #[regex(r#"'(?:[^']|\\')*'"#, |lex| Cow::Borrowed(&lex.slice()[1..lex.slice().len()-1]))]
    String(Cow<'a, str>),

    // Operators
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    DashDash,
    #[token("**")]
    MulMul,
    #[token("//")]
    DivDiv,
    #[regex("[Tt]rue")]
    True,
    #[regex("[Ff]alse")]
    False,
    #[token("None")]
    None,

    // Keywords
    #[token("or")]
    LogicalOr,
    #[token("and")]
    LogicalAnd,
    #[token("not")]
    LogicalNot,
    #[token("in")]
    In,
    #[token("is")]
    Is,
    #[token("for")]
    For,
    #[token("endfor")]
    EndFor,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("elif")]
    ElIf,
    #[token("endif")]
    EndIf,
    #[token("block")]
    Block,
    #[token("endblock")]
    EndBlock,
    #[token("extends")]
    Extends,
    #[token("macro")]
    Macro,
    #[token("endmacro")]
    EndMacro,
    #[token("call")]
    Call,
    #[token("endcall")]
    EndCall,
    #[token("filter")]
    Filter,
    #[token("endfilter")]
    EndFilter,
    #[token("set")]
    Set,
    #[token("endset")]
    EndSet,
    #[token("include")]
    Include,
    #[token("import")]
    Import,
    #[token("recursive")]
    Recursive,
    #[token("scoped")]
    Scoped,
    #[token("with")]
    With,
    #[token("endwith")]
    EndWith,
    #[token("without")]
    Without,
    #[token("ignore")]
    Ignore,
    #[token("missing")]
    Missing,
    #[token("context")]
    Context,
    #[token("from")]
    From,
    #[token("as")]
    As,
    #[token("do")]
    Do,

    // Template control
    #[token("{#")]
    CommentBegin,
    #[token("#}")]
    CommentEnd,
    #[token("{{% raw %}}")]
    RawBegin,
    #[token("{{% endraw %}}")]
    RawEnd,
    #[token("{%")]
    StmtBegin,
    #[token("%}")]
    StmtEnd,
    #[token("{{")]
    ExprBegin,
    #[token("}}")]
    ExprEnd,
}

pub struct PeekableLexer<'source, T: Logos<'source>> {
    lexer: Lexer<'source, T>,
    peeked: Option<Option<T>>,
}

impl<'source, T> PeekableLexer<'source, T>
where
    T: Logos<'source>,
{
    pub fn new(lexer: Lexer<'source, T>) -> Self {
        Self {
            lexer,
            peeked: None,
        }
    }
    #[inline]
    pub fn peek(&mut self) -> Option<&T> {
        let lexer = &mut self.lexer;
        self.peeked.get_or_insert_with(|| lexer.next()).as_ref()
    }

    #[inline]
    pub fn span(&self) -> core::ops::Range<usize> {
        self.lexer.span()
    }
}

impl<'source, T> Iterator for PeekableLexer<'source, T>
where
    T: Logos<'source>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.take() {
            Some(token) => token,
            None => self.lexer.next(),
        }
    }
}

#[test]
fn lex_numbers() {
    let tokens: Vec<_> = Token::lexer("1 42 -100 3.18 -77.77").collect();
    assert_eq!(
        tokens,
        &[
            Token::IntegerNum(1),
            Token::IntegerNum(42),
            Token::Minus,
            Token::IntegerNum(100),
            Token::FloatNum(3.18),
            Token::Minus,
            Token::FloatNum(77.77),
        ]
    );
}

#[test]
fn lex_strings() {
    let tokens: Vec<_> = Token::lexer("\"some string\" \"\"").collect();
    assert_eq!(
        tokens,
        &[
            Token::String(std::borrow::Cow::Borrowed("some string")),
            Token::String(std::borrow::Cow::Borrowed("")),
        ]
    );
}

#[test]
fn lex_math() {
    let tokens: Vec<_> = Token::lexer("(2 + 3 * (5 - 1) + 2 ** 3 / 16) % 5").collect();
    assert_eq!(
        tokens,
        &[
            Token::LBracket,
            Token::IntegerNum(2),
            Token::Plus,
            Token::IntegerNum(3),
            Token::Mul,
            Token::LBracket,
            Token::IntegerNum(5),
            Token::Minus,
            Token::IntegerNum(1),
            Token::RBracket,
            Token::Plus,
            Token::IntegerNum(2),
            Token::MulMul,
            Token::IntegerNum(3),
            Token::Div,
            Token::IntegerNum(16),
            Token::RBracket,
            Token::Percent,
            Token::IntegerNum(5),
        ]
    );
}
