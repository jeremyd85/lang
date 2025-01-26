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
    Eof,
    Unexpected,
    Newline,
    Whitespace,
    Identifier

}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    pos: u32,
    value: Spur
}

impl Token {
    fn new(kind: TokenKind, pos: u32) -> Token {
        let sym = Spur::default();
        Token {
            kind, pos, value: sym,
        }
    }
}

/*
assignment = identifier eq expression

expression = number

Node {
    kind NodeKind
    lhs u32
    rhs u32
    data u32
}

FunctionDef

Parser {
    function_defs Vec<FuncionDef>
}
*/

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

    fn consume_identifier(&mut self, pos: u32) -> Token {
        let mut identifier = String::new();
        identifier.push(self.peek());
        let mut token = Token::new(TokenKind::Identifier, pos);
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            let (_, ch) = self.take_char();
            identifier.push(ch);
        }
        let symbol = self.interner.get_or_intern(identifier);
        token.value = symbol;
        token
    }

    fn get_token(&mut self, pos: u32, ch: char) -> Option<Token> {
        let token = match ch {
            '\n' => {
                Some(Token::new(TokenKind::Newline, pos))
            },
            ch if ch.is_ascii_whitespace() => {
                while self.peek().is_ascii_whitespace() && self.peek() != '\n' {
                    self.consume();
                }
                Some(Token::new(TokenKind::Whitespace, pos))
            },
            '=' => {
                if self.peek() == '=' {
                    self.consume();
                    Some(Token::new(TokenKind::EqEq, pos))
                } else {
                    Some(Token::new(TokenKind::Eq, pos))
                }
            },
            'A'..='z' => {
                Some(self.consume_identifier(pos))
            },
            '0'..='9' => {
                None
            },
            '-' => {
                None
            },
            _ => None
        };
        return token
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut unrecognized = None;
        while self.peek() != '\0' {
            let (pos, ch) = self.take_char();
            let token = self.get_token(pos, ch);
            if let Some(token) = token {
                if let Some(finished_unrecognized) = unrecognized {
                    tokens.push(finished_unrecognized);
                    unrecognized = None;
                }
                tokens.push(token);
            } else if unrecognized.is_none() {
                unrecognized = Some(Token::new(TokenKind::Unexpected, pos));
            }
        }
        if let Some(finished_unrecognized) = unrecognized {
            tokens.push(finished_unrecognized);
        }
        tokens.push(Token::new(TokenKind::Eof, self.pos));
        return tokens;
    }
}


#[test]
pub fn test_unrecognized() {
    let mut interner = Arc::new(ThreadedRodeo::default());
    let mut lexer = Lexer::new("==Hg_kjdg =\n", &mut interner);
    assert_eq!(lexer.lex(), vec![
        Token::new(TokenKind::EqEq, 0),
        Token::new(TokenKind::Identifier, 2),
        Token::new(TokenKind::Whitespace, 9),
        Token::new(TokenKind::Eq, 10),
        Token::new(TokenKind::Newline, 11),
        Token::new(TokenKind::Eof, 12)
        ])
}

#[test]
fn test_match_tokens() {
    let mut interner = Arc::new(ThreadedRodeo::default());
    let mut lexer = Lexer::new("== =\n", &mut interner);
    let tokenKinds: Vec<TokenKind> = lexer.lex().iter().map(|token| token.kind).collect();
    assert_eq!(tokenKinds, vec![
        TokenKind::EqEq,
        TokenKind::Whitespace,
        TokenKind::Eq,
        TokenKind::Newline,
        TokenKind::Eof
        ])
}

#[test]
fn ensure_size() {
    assert_eq!(size_of::<Token>(), 24);
}
