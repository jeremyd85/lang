use lasso::{ThreadedRodeo, Spur};
use std::sync::Arc;

#[derive(Debug)]
pub struct Lexer<'a> {
    content: &'a str,
    pos: u32,
    interner: &'a mut Arc<ThreadedRodeo>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Eq,
    EqEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Eof,
    Unexpected,
    Newline,
    Whitespace,
    Identifier,
    Number,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Not,
    NotEq,
    BitOr,
    LogicalOr,
    BitAnd,
    LogicalAnd,
    LParenthesis,
    RParenthesis,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: u32,
    pub value: Spur
}

impl Token {
    fn new(kind: TokenKind, pos: u32) -> Token {
        let sym = Spur::default();
        Token {
            kind, pos, value: sym,
        }
    }
}

impl<'a> Lexer<'a> {
    
    pub fn new(content: &'a str, interner: &'a mut Arc<ThreadedRodeo>) -> Self {
        Lexer {
            content,
            pos: 0,
            interner
        }
    }

    fn take_char(&mut self) -> (u32, char) {
        if let Some(ch) = self.content.chars().nth(self.pos as usize) {
            self.pos += 1;
            return (self.pos - 1, ch);
        }
        (self.pos, '\0')
    }

    fn peek(&self) -> char {
        if let Some(ch) = self.content.chars().nth(self.pos as usize) {
            return ch
        }
        '\0'
    }

    fn consume(&mut self) {
        self.pos += 1;
    }

    fn consume_identifier(&mut self, pos: u32, ch: char) -> Token {
        self.consume();
        let mut identifier = String::from(ch);
        let mut token = Token::new(TokenKind::Identifier, pos);
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            let (_, ch) = self.take_char();
            identifier.push(ch);
        }
        token.value = self.interner.get_or_intern(identifier);
        token
    }

    fn consume_whitespace(&mut self, pos: u32, ch: char) -> Token {
        self.consume();
        let mut token = Token::new(TokenKind::Whitespace, pos);
        let mut whitespace = String::from(ch);
        while self.peek().is_ascii_whitespace() && self.peek() != '\n' {
            let (_, ch) = self.take_char();
            whitespace.push(ch);
        }
        token.value = self.interner.get_or_intern(whitespace);
        token
    }

    fn consume_number(&mut self, pos: u32, ch: char) -> Token {
        self.consume();
        let mut number = String::from(ch);
        let mut token = Token::new(TokenKind::Number, pos);
        let mut consumed_dot = false;
        while self.peek().is_ascii_digit() || self.peek() == '.' {
            if self.peek() == '.' {
                if consumed_dot {
                    break;
                }
                consumed_dot = true;
            }
            let (_, ch) = self.take_char();
            number.push(ch);
        }
        token.value = self.interner.get_or_intern(number);
        token
    }

    fn consume_single(&mut self, pos: u32, kind: TokenKind) -> Token {
        self.consume();
        Token::new(kind, pos)
    }

    fn finish_maybe_two_symbol(&mut self, pos: u32, default_kind: TokenKind, kind: TokenKind, second: char) -> Token {
        self.consume();
        if self.peek() == second {
            self.consume();
            return Token::new(kind, pos)
        }
        Token::new(default_kind, pos)
    }

    fn get_token(&mut self, pos: u32, ch: char) -> Option<Token> {
        match ch {
            '\n' => Some(self.consume_single(pos, TokenKind::Newline)),
            '.' => Some(self.consume_single(pos, TokenKind::Dot)),
            '+' => Some(self.consume_single(pos, TokenKind::Plus)),
            '-' => Some(self.consume_single(pos, TokenKind::Minus)),
            '*' => Some(self.consume_single(pos, TokenKind::Star)),
            '/' => Some(self.consume_single(pos, TokenKind::Slash)),
            '(' => Some(self.consume_single(pos, TokenKind::LParenthesis)),
            ')' => Some(self.consume_single(pos, TokenKind::RParenthesis)),
            '=' => Some(self.finish_maybe_two_symbol(
                pos, 
                TokenKind::Eq, 
                TokenKind::EqEq, 
                '=')
            ),
            '>' => Some(self.finish_maybe_two_symbol(
                pos, 
                TokenKind::Gt, 
                TokenKind::GtEq, 
                '=')
            ),
            '<' => Some(self.finish_maybe_two_symbol(
                pos, 
                TokenKind::Lt, 
                TokenKind::LtEq, 
                '=')
            ),
            '!' => Some(self.finish_maybe_two_symbol(
                pos, 
                TokenKind::Not, 
                TokenKind::NotEq, 
                '=')
            ),
            '|' => Some(self.finish_maybe_two_symbol(
                pos, 
                TokenKind::BitOr, 
                TokenKind::LogicalOr, 
                '|')
            ),
            '&' => Some(self.finish_maybe_two_symbol(
                pos, 
                TokenKind::BitAnd, 
                TokenKind::LogicalAnd, 
                '|')
            ),
            ch if ch.is_ascii_whitespace() => Some(self.consume_whitespace(pos, ch)),
            'A'..='z' => Some(self.consume_identifier(pos, ch)),
            '0'..='9' => Some(self.consume_number(pos, ch)),
            _ => None
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut unrecognized: Option<Token> = None;
        let mut unrecognized_str = String::new();
        while self.peek() != '\0' {
            let ch = self.peek();
            let pos = self.pos;
            let token = self.get_token(pos, ch);
            if let Some(token) = token {
                if let Some(mut finished_unrecognized) = unrecognized {
                    finished_unrecognized.value = self.interner.get_or_intern(
                        unrecognized_str.clone()
                    );
                    tokens.push(finished_unrecognized);
                    unrecognized = None;
                    unrecognized_str.clear();
                }
                tokens.push(token);
            } else if unrecognized.is_none() {
                self.consume();
                unrecognized_str.push(ch);
                unrecognized = Some(Token::new(TokenKind::Unexpected, pos));
            } else {
                self.consume();
                unrecognized_str.push(ch);
            }
        }
        if let Some(mut finished_unrecognized) = unrecognized {
            finished_unrecognized.value = self.interner.get_or_intern(
                unrecognized_str.clone()
            );
            tokens.push(finished_unrecognized);
        }
        tokens.push(Token::new(TokenKind::Eof, self.pos));
        return tokens;
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct TokenStream<'a> {
    tokens: &'a [Token],
    pos: u32,
}

impl TokenStream<'_> {

    pub fn new(tokens: &[Token]) -> TokenStream {
        TokenStream {
            tokens,
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos as usize)
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn consume(&mut self, token_kind: TokenKind) -> Option<&Token> {
        self.pos += 1;
        let token: Option<&Token> = self.peek();
        if let Some(t) = token {
            if t.kind == token_kind {
                return Some(t);
            }
            return None;
        }
        None
    }
}


#[test]
pub fn test_unrecognized() {
    let mut interner = Arc::new(ThreadedRodeo::default());
    let mut lexer = Lexer::new("==Hg_kjdg =\n", &mut interner);
    assert_eq!(lexer.lex().iter().map(|token| token.kind).collect::<Vec<TokenKind>>(), vec![
            TokenKind::EqEq,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Eq,
            TokenKind::Newline,
            TokenKind::Eof,
        ]
    );
}

#[test]
pub fn assignment() {
    let mut interner = Arc::new(ThreadedRodeo::default());
    let mut lexer = Lexer::new("abc = 123.5.6", &mut interner);
    let tokens = lexer.lex();
    let token_kinds: Vec<TokenKind> = tokens.iter().map(|token| token.kind).collect();
    assert_eq!(token_kinds, vec![
        TokenKind::Identifier,
        TokenKind::Whitespace,
        TokenKind::Eq,
        TokenKind::Whitespace,
        TokenKind::Number,
        TokenKind::Unexpected,
        TokenKind::Eof
        ]);
    assert_eq!(interner.resolve(&tokens[0].value), "abc");
    assert_eq!(interner.resolve(&tokens[4].value), "123.5");
}

#[test]
pub fn consume_whole_identifier() {
    let mut interner = Arc::new(ThreadedRodeo::default());
    let mut lexer = Lexer::new("abc", &mut interner);
    let tokens = lexer.lex();
    let token = tokens.get(0).unwrap();
    assert_eq!(token.value, interner.get_or_intern("abc"));
}

#[test]
fn test_match_tokens() {
    let mut interner = Arc::new(ThreadedRodeo::default());
    let mut lexer = Lexer::new("== =\n", &mut interner);
    let token_kinds: Vec<TokenKind> = lexer.lex().iter().map(|token| token.kind).collect();
    assert_eq!(token_kinds, vec![
        TokenKind::EqEq,
        TokenKind::Whitespace,
        TokenKind::Eq,
        TokenKind::Newline,
        TokenKind::Eof
        ])
}

#[test]
fn ensure_size() {
    assert_eq!(size_of::<Token>(), 12);
}
