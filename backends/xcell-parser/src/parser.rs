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
    fn current(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or_else(|| Token::eof(0))
    }

    /// 前进一个 Token
    fn advance(&mut self) -> Token {
        let token = self.current();
        self.pos += 1;
        token
    }

    /// 期望特定类型的 Token
    fn expect(&mut self, kind: TokenKind) -> ParseResult<Token> {
        let token = self.current();
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
        self.parse_prefix_type()
    }

    /// 解析前缀类型（处理 `&`、`@`、`@@` 等前缀修饰符）
    fn parse_prefix_type(&mut self) -> ParseResult<TypeExpr> {
        match self.current().kind {
            // 引用类型 `&TableName`
            TokenKind::Ampersand => {
                self.advance();
                let name_token = self.expect(TokenKind::Identifier)?;
                Ok(TypeExpr::Reference { target: name_token.text })
            }
            // 独一类型 `@T` 或主键类型 `@@T`
            TokenKind::At => {
                self.advance();
                // 检查是否为主键 `@@T`
                if self.current().kind == TokenKind::At {
                    self.advance();
                    let inner = self.parse_primary_type()?;
                    Ok(TypeExpr::PrimaryKey { inner: Box::new(inner) })
                } else {
                    let inner = self.parse_primary_type()?;
                    Ok(TypeExpr::Unique { inner: Box::new(inner) })
                }
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
            
            // 特殊处理 Vec<T>
            if name.eq_ignore_ascii_case("Vec") && args.len() == 1 {
                return Ok(TypeExpr::Vec { element: Box::new(args.remove(0)) });
            }
            
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
        assert!(matches!(parse("f32").unwrap(), TypeExpr::Primitive(PrimitiveType::F32)));
    }

    #[test]
    fn test_reference_type() {
        let ty = parse("&Item").unwrap();
        assert!(matches!(ty, TypeExpr::Reference { target } if target == "Item"));
        
        let ty = parse("&Quality").unwrap();
        assert!(matches!(ty, TypeExpr::Reference { target } if target == "Quality"));
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
    fn test_vec_type() {
        let ty = parse("Vec<i32>").unwrap();
        match ty {
            TypeExpr::Vec { element } => {
                assert!(matches!(*element, TypeExpr::Primitive(PrimitiveType::I32)));
            }
            _ => panic!("Expected Vec type"),
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
    fn test_unique_type() {
        let ty = parse("@i32").unwrap();
        match ty {
            TypeExpr::Unique { inner } => {
                assert!(matches!(*inner, TypeExpr::Primitive(PrimitiveType::I32)));
            }
            _ => panic!("Expected Unique type"),
        }
    }

    #[test]
    fn test_primary_key_type() {
        let ty = parse("@@i32").unwrap();
        match ty {
            TypeExpr::PrimaryKey { inner } => {
                assert!(matches!(*inner, TypeExpr::Primitive(PrimitiveType::I32)));
            }
            _ => panic!("Expected PrimaryKey type"),
        }
    }

    #[test]
    fn test_unique_string() {
        let ty = parse("@string").unwrap();
        match ty {
            TypeExpr::Unique { inner } => {
                assert!(matches!(*inner, TypeExpr::Primitive(PrimitiveType::String)));
            }
            _ => panic!("Expected Unique type"),
        }
    }

    #[test]
    fn test_primary_key_string() {
        let ty = parse("@@string").unwrap();
        match ty {
            TypeExpr::PrimaryKey { inner } => {
                assert!(matches!(*inner, TypeExpr::Primitive(PrimitiveType::String)));
            }
            _ => panic!("Expected PrimaryKey type"),
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
    fn test_named_type() {
        let ty = parse("QualityType").unwrap();
        match ty {
            TypeExpr::Named(name) => {
                assert_eq!(name, "QualityType");
            }
            _ => panic!("Expected Named type"),
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(parse("i32").unwrap().to_string(), "i32");
        assert_eq!(parse("&Item").unwrap().to_string(), "&Item");
        assert_eq!(parse("[i32]").unwrap().to_string(), "[i32]");
        assert_eq!(parse("[i32; 5]").unwrap().to_string(), "[i32; 5]");
        assert_eq!(parse("Vec<i32>").unwrap().to_string(), "Vec<i32>");
        assert_eq!(parse("@i32").unwrap().to_string(), "@i32");
        assert_eq!(parse("@@string").unwrap().to_string(), "@@string");
    }
}
