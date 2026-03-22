use crate::error::{ParseError, ParseErrorKind, ParseResult};
use crate::lexer::{Token, TokenKind};
use crate::ast::{PrimitiveType, TypeExpr};

/// 类型表达式解析器
pub struct TypeParser {
    /// Token 列表
    tokens: Vec<Token>,
    /// 当前位置
    pos: usize,
}

impl TypeParser {
    /// 创建新的解析器
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// 解析类型表达式
    pub fn parse_type_expr(&mut self) -> ParseResult<TypeExpr> {
        if self.tokens.is_empty() {
            return Err(ParseError::new(ParseErrorKind::EmptyInput, 0));
        }

        let result = self.parse_type()?;
        
        // 确保所有 token 都被消费
        if self.pos < self.tokens.len() && self.tokens[self.pos].kind != TokenKind::Eof {
            return Err(ParseError::unexpected_token(
                "输入结束",
                Some(&self.tokens[self.pos].text),
                self.tokens[self.pos].start,
            ));
        }

        Ok(result)
    }

    /// 获取当前 Token
    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::eof(0))
    }

    /// 前进一个 Token
    fn advance(&mut self) -> &Token {
        let token = self.current();
        self.pos += 1;
        token
    }

    /// 期望特定类型的 Token
    fn expect(&mut self, kind: TokenKind) -> ParseResult<Token> {
        let token = self.current().clone();
        if token.kind == kind {
            self.advance();
            Ok(token)
        } else {
            Err(ParseError::unexpected_token(
                &kind.to_string(),
                Some(&token.kind.to_string()),
                token.start,
            ))
        }
    }

    /// 解析类型
    fn parse_type(&mut self) -> ParseResult<TypeExpr> {
        self.parse_postfix_type()
    }

    /// 解析后缀类型（处理 `?` 等后缀修饰符）
    fn parse_postfix_type(&mut self) -> ParseResult<TypeExpr> {
        let mut ty = self.parse_prefix_type()?;

        loop {
            match self.current().kind {
                TokenKind::Question => {
                    self.advance();
                    ty = TypeExpr::Optional(Box::new(ty));
                }
                _ => break,
            }
        }

        Ok(ty)
    }

    /// 解析前缀类型（处理 `&`、`*`、`@`、`@@` 等前缀修饰符）
    fn parse_prefix_type(&mut self) -> ParseResult<TypeExpr> {
        match self.current().kind {
            TokenKind::Ampersand => {
                self.advance();
                let name_token = self.expect(TokenKind::Identifier)?;
                Ok(TypeExpr::Reference { target: name_token.text })
            }
            TokenKind::Asterisk => {
                self.advance();
                let inner = self.parse_prefix_type()?;
                Ok(TypeExpr::Pointer(Box::new(inner)))
            }
            TokenKind::At => {
                self.advance();
                let is_primary = if self.current().kind == TokenKind::At {
                    self.advance();
                    true
                } else {
                    false
                };
                let inner = self.parse_prefix_type()?;
                Ok(TypeExpr::Unique {
                    inner: Box::new(inner),
                    is_primary,
                })
            }
            _ => self.parse_primary_type(),
        }
    }

    /// 解析基本类型
    fn parse_primary_type(&mut self) -> ParseResult<TypeExpr> {
        match self.current().kind {
            TokenKind::LeftBracket => self.parse_list_or_array(),
            TokenKind::LeftParen => self.parse_tuple(),
            TokenKind::Identifier => self.parse_named_or_generic(),
            _ => Err(ParseError::unexpected_token(
                "类型表达式",
                Some(&self.current().kind.to_string()),
                self.current().start,
            )),
        }
    }

    /// 解析列表或固定数组 `[T]` 或 `[T; N]`
    fn parse_list_or_array(&mut self) -> ParseResult<TypeExpr> {
        self.expect(TokenKind::LeftBracket)?;
        let element = self.parse_type()?;

        if self.current().kind == TokenKind::Semicolon {
            self.advance();
            let len_token = self.expect(TokenKind::Integer)?;
            let length: usize = len_token.text.parse().map_err(|_| {
                ParseError::new(ParseErrorKind::InvalidNumberLiteral(len_token.text.clone()), len_token.start)
            })?;
            self.expect(TokenKind::RightBracket)?;
            Ok(TypeExpr::FixedArray {
                element: Box::new(element),
                length,
            })
        } else {
            self.expect(TokenKind::RightBracket)?;
            Ok(TypeExpr::List {
                element: Box::new(element),
            })
        }
    }

    /// 解析元组 `(T1, T2, ...)`
    fn parse_tuple(&mut self) -> ParseResult<TypeExpr> {
        self.expect(TokenKind::LeftParen)?;
        let mut elements = Vec::new();

        if self.current().kind != TokenKind::RightParen {
            elements.push(self.parse_type()?);
            while self.current().kind == TokenKind::Comma {
                self.advance();
                elements.push(self.parse_type()?);
            }
        }

        self.expect(TokenKind::RightParen)?;
        Ok(TypeExpr::Tuple(elements))
    }

    /// 解析命名类型或泛型类型
    fn parse_named_or_generic(&mut self) -> ParseResult<TypeExpr> {
        let name_token = self.expect(TokenKind::Identifier)?;
        let name = name_token.text;

        // 检查是否为原始类型
        if let Some(prim) = PrimitiveType::from_str(&name) {
            return Ok(TypeExpr::Primitive(prim));
        }

        // 检查是否为 ref<...>
        if name.eq_ignore_ascii_case("ref") {
            self.expect(TokenKind::LeftAngle)?;
            let target_token = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::RightAngle)?;
            return Ok(TypeExpr::Ref { target: target_token.text });
        }

        // 检查是否为泛型 `Name<T1, T2, ...>`
        if self.current().kind == TokenKind::LeftAngle {
            self.advance();
            let mut args = Vec::new();

            if self.current().kind != TokenKind::RightAngle {
                args.push(self.parse_type()?);
                while self.current().kind == TokenKind::Comma {
                    self.advance();
                    args.push(self.parse_type()?);
                }
            }

            self.expect(TokenKind::RightAngle)?;
            return Ok(TypeExpr::Generic { name, args });
        }

        // 普通命名类型
        Ok(TypeExpr::Named(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> ParseResult<TypeExpr> {
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect::<Result<_, _>>()?;
        let mut parser = TypeParser::new(tokens);
        parser.parse_type_expr()
    }

    #[test]
    fn test_primitive_types() {
        assert!(matches!(parse("i32").unwrap(), TypeExpr::Primitive(PrimitiveType::I32)));
        assert!(matches!(parse("bool").unwrap(), TypeExpr::Primitive(PrimitiveType::Bool)));
        assert!(matches!(parse("string").unwrap(), TypeExpr::Primitive(PrimitiveType::String)));
    }

    #[test]
    fn test_reference_type() {
        let ty = parse("&Item").unwrap();
        assert!(matches!(ty, TypeExpr::Reference { target } if target == "Item"));
    }

    #[test]
    fn test_ref_generic() {
        let ty = parse("ref<Monster>").unwrap();
        assert!(matches!(ty, TypeExpr::Ref { target } if target == "Monster"));
    }

    #[test]
    fn test_list_type() {
        let ty = parse("[i32]").unwrap();
        match ty {
            TypeExpr::List { element } => {
                assert!(matches!(*element, TypeExpr::Primitive(PrimitiveType::I32)));
            }
            _ => panic!("Expected List type"),
        }
    }

    #[test]
    fn test_fixed_array() {
        let ty = parse("[i32; 5]").unwrap();
        match ty {
            TypeExpr::FixedArray { element, length } => {
                assert!(matches!(*element, TypeExpr::Primitive(PrimitiveType::I32)));
                assert_eq!(length, 5);
            }
            _ => panic!("Expected FixedArray type"),
        }
    }

    #[test]
    fn test_nested_list() {
        let ty = parse("[[i32]]").unwrap();
        match ty {
            TypeExpr::List { element } => {
                assert!(matches!(*element, TypeExpr::List { .. }));
            }
            _ => panic!("Expected nested List type"),
        }
    }

    #[test]
    fn test_reference_list() {
        let ty = parse("[&Item]").unwrap();
        match ty {
            TypeExpr::List { element } => {
                assert!(matches!(*element, TypeExpr::Reference { target } if target == "Item"));
            }
            _ => panic!("Expected List of Reference type"),
        }
    }

    #[test]
    fn test_optional_type() {
        let ty = parse("i32?").unwrap();
        match ty {
            TypeExpr::Optional(inner) => {
                assert!(matches!(*inner, TypeExpr::Primitive(PrimitiveType::I32)));
            }
            _ => panic!("Expected Optional type"),
        }
    }

    #[test]
    fn test_unique_type() {
        let ty = parse("@i32").unwrap();
        match ty {
            TypeExpr::Unique { inner, is_primary } => {
                assert!(matches!(*inner, TypeExpr::Primitive(PrimitiveType::I32)));
                assert!(!is_primary);
            }
            _ => panic!("Expected Unique type"),
        }
    }

    #[test]
    fn test_primary_key_type() {
        let ty = parse("@@i32").unwrap();
        match ty {
            TypeExpr::Unique { inner, is_primary } => {
                assert!(matches!(*inner, TypeExpr::Primitive(PrimitiveType::I32)));
                assert!(is_primary);
            }
            _ => panic!("Expected Unique type with primary key"),
        }
    }

    #[test]
    fn test_generic_type() {
        let ty = parse("HashMap<string, i32>").unwrap();
        match ty {
            TypeExpr::Generic { name, args } => {
                assert_eq!(name, "HashMap");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected Generic type"),
        }
    }

    #[test]
    fn test_tuple_type() {
        let ty = parse("(i32, string, bool)").unwrap();
        match ty {
            TypeExpr::Tuple(elements) => {
                assert_eq!(elements.len(), 3);
            }
            _ => panic!("Expected Tuple type"),
        }
    }

    #[test]
    fn test_complex_nested_type() {
        let ty = parse("[&Item]?").unwrap();
        match ty {
            TypeExpr::Optional(inner) => {
                match *inner {
                    TypeExpr::List { element } => {
                        assert!(matches!(*element, TypeExpr::Reference { target } if target == "Item"));
                    }
                    _ => panic!("Expected List inside Optional"),
                }
            }
            _ => panic!("Expected Optional type"),
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(parse("i32").unwrap().to_string(), "I32");
        assert_eq!(parse("&Item").unwrap().to_string(), "&Item");
        assert_eq!(parse("[i32]").unwrap().to_string(), "[i32]");
        assert_eq!(parse("[i32; 5]").unwrap().to_string(), "[i32; 5]");
        assert_eq!(parse("ref<Monster>").unwrap().to_string(), "ref<Monster>");
    }
}
