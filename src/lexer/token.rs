#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    block: String,
}

impl Token {
    pub fn new(kind: TokenKind, block: String) -> Self {
        Token { kind, block }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Local,
    Then,
    If,
    Else,
    End,
    Do,
    Gt,
    OpeningParenthesis,
    ClosingParenthesis,
    Assignement,
    Equality,
    String(char),
    Table,
    Number,
}
