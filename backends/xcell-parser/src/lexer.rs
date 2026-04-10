use std::iter::Peekable;
use std::str::Chars;

use crate::error::{ParseError, ParseResult};

/// Token 类型
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// 标识符（类型名、字段名等）
    Identifier,
    /// 整数字面量
    Integer,
    /// 字符串字面量
    String,
    /// 左方括号 `[`
    LeftBracket,
    /// 右方括号 `]`
    RightBracket,
    /// 左尖括号 `<`
    LeftAngle,
    /// 右尖括号 `>`
    RightAngle,
    /// 左圆括号 `(`
    LeftParen,
    /// 右圆括号 `)`
    RightParen,
    /// 左花括号 `{`
    LeftBrace,
    /// 右花括号 `}`
    RightBrace,
    /// 逗号 `,`
    Comma,
    /// 分号 `;`
    Semicolon,
    /// 冒号 `:`
    Colon,
    /// 引用符号 `&`
    Ampersand,
    /// 星号 `*`
    Asterisk,
    /// 问号 `?`
    Question,
    /// 独一标记 `@`
    At,
    /// 输入结束
    Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Identifier => write!(f, "标识符"),
            TokenKind::Integer => write!(f, "整数"),
            TokenKind::String => write!(f, "字符串"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::LeftAngle => write!(f, "<"),
            TokenKind::RightAngle => write!(f, ">"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Question => write!(f, "?"),
            TokenKind::At => write!(f, "@"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

/// Token 结构
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Token 类型
    pub kind: TokenKind,
    /// Token 文本
    pub text: String,
    /// 起始位置（字节偏移）
    pub start: usize,
    /// 结束位置（字节偏移）
    pub end: usize,
}

impl Token {
    /// 创建新的 Token
    pub fn new(kind: TokenKind, text: String, start: usize, end: usize) -> Self {
        Self { kind, text, start, end }
    }

    /// 创建 EOF Token
    pub fn eof(position: usize) -> Self {
        Self::new(TokenKind::Eof, String::new(), position, position)
    }
}

/// 词法分析器
pub struct Lexer<'a> {
    /// 输入字符迭代器
    chars: Peekable<Chars<'a>>,
    /// 当前位置
    position: usize,
    /// 输入字符串
    input: &'a str,
}

impl<'a> Lexer<'a> {
    /// 创建新的词法分析器
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            position: 0,
            input,
        }
    }

    /// 查看下一个字符但不消费
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// 消费并返回下一个字符
    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(ch) = c {
            self.position += ch.len_utf8();
        }
        c
    }

    /// 跳过空白字符
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// 读取标识符
    fn read_identifier(&mut self, start: usize) -> Token {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let text = self.input[start..self.position].to_string();
        Token::new(TokenKind::Identifier, text, start, self.position)
    }

    /// 读取数字
    fn read_number(&mut self, start: usize) -> Token {
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        let text = self.input[start..self.position].to_string();
        Token::new(TokenKind::Integer, text, start, self.position)
    }

    /// 读取字符串字面量（双引号包围）
    fn read_string(&mut self, start: usize) -> ParseResult<Token> {
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                let text = self.input[start..self.position].to_string();
                return Ok(Token::new(TokenKind::String, text, start, self.position));
            }
            self.advance();
        }
        Err(ParseError::new(
            crate::error::ParseErrorKind::InvalidSyntax("未闭合的字符串".to_string()),
            start,
        ))
    }

    /// 获取下一个 Token
    pub fn next_token(&mut self) -> ParseResult<Token> {
        self.skip_whitespace();

        let start = self.position;

        match self.advance() {
            None => Ok(Token::eof(start)),
            Some(c) => {
                match c {
                    '[' => Ok(Token::new(TokenKind::LeftBracket, "[".to_string(), start, self.position)),
                    ']' => Ok(Token::new(TokenKind::RightBracket, "]".to_string(), start, self.position)),
                    '<' => Ok(Token::new(TokenKind::LeftAngle, "<".to_string(), start, self.position)),
                    '>' => Ok(Token::new(TokenKind::RightAngle, ">".to_string(), start, self.position)),
                    '(' => Ok(Token::new(TokenKind::LeftParen, "(".to_string(), start, self.position)),
                    ')' => Ok(Token::new(TokenKind::RightParen, ")".to_string(), start, self.position)),
                    '{' => Ok(Token::new(TokenKind::LeftBrace, "{".to_string(), start, self.position)),
                    '}' => Ok(Token::new(TokenKind::RightBrace, "}".to_string(), start, self.position)),
                    ',' => Ok(Token::new(TokenKind::Comma, ",".to_string(), start, self.position)),
                    ';' => Ok(Token::new(TokenKind::Semicolon, ";".to_string(), start, self.position)),
                    ':' => Ok(Token::new(TokenKind::Colon, ":".to_string(), start, self.position)),
                    '&' => Ok(Token::new(TokenKind::Ampersand, "&".to_string(), start, self.position)),
                    '*' => Ok(Token::new(TokenKind::Asterisk, "*".to_string(), start, self.position)),
                    '?' => Ok(Token::new(TokenKind::Question, "?".to_string(), start, self.position)),
                    '@' => Ok(Token::new(TokenKind::At, "@".to_string(), start, self.position)),
                    '"' => self.read_string(start),
                    c if c.is_ascii_digit() => Ok(self.read_number(start)),
                    c if c.is_alphabetic() || c == '_' => Ok(self.read_identifier(start)),
                    c => Err(ParseError::new(
                        crate::error::ParseErrorKind::InvalidSyntax(format!("意外的字符: {}", c)),
                        start,
                    )),
                }
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = ParseResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(token) if token.kind == TokenKind::Eof => None,
            Ok(token) => Some(Ok(token)),
            Err(e) => Some(Err(e)),
        }
    }
}
