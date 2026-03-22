use crate::error::{ParseError, ParseErrorKind, ParseResult};
use crate::lexer::{Token, TokenKind};
use crate::ast::{FieldConstraint, FieldExpr, FieldMeta, MetaExpr, PrimitiveType, TableKind, TypeExpr, TypeMeta};

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

    /// 检查是否到达末尾
    fn is_at_end(&self) -> bool {
        self.current().kind == TokenKind::Eof
    }

    /// 解析类型表达式
    pub fn parse_type_expr(&mut self) -> ParseResult<TypeExpr> {
        if self.tokens.is_empty() {
            return Err(ParseError::new(ParseErrorKind::EmptyInput, 0));
        }

        let result = self.parse_type()?;
        
        if !self.is_at_end() {
            return Err(ParseError::unexpected_token(
                "输入结束",
                Some(&self.current().text),
                self.current().start,
            ));
        }

        Ok(result)
    }

    /// 解析类型
    fn parse_type(&mut self) -> ParseResult<TypeExpr> {
        self.parse_primary_type()
    }

    /// 解析基本类型
    fn parse_primary_type(&mut self) -> ParseResult<TypeExpr> {
        match self.current().kind {
            TokenKind::Ampersand => self.parse_reference_type(),
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

    /// 解析引用类型 `&TableName`
    fn parse_reference_type(&mut self) -> ParseResult<TypeExpr> {
        self.expect(TokenKind::Ampersand)?;
        let name_token = self.expect(TokenKind::Identifier)?;
        Ok(TypeExpr::Reference { target: name_token.text })
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

    /// 解析字段表达式（字段名 + 可选约束）
    pub fn parse_field_expr(&mut self) -> ParseResult<FieldExpr> {
        if self.tokens.is_empty() {
            return Err(ParseError::new(ParseErrorKind::EmptyInput, 0));
        }

        let constraint = match self.current().kind {
            // @@field_name - 主键约束
            TokenKind::At => {
                self.advance();
                if self.current().kind == TokenKind::At {
                    self.advance();
                    Some(FieldConstraint::Primary)
                } else {
                    Some(FieldConstraint::Unique)
                }
            }
            _ => None,
        };

        let name_token = self.expect(TokenKind::Identifier)?;
        
        if !self.is_at_end() {
            return Err(ParseError::unexpected_token(
                "输入结束",
                Some(&self.current().text),
                self.current().start,
            ));
        }

        Ok(FieldExpr {
            name: name_token.text,
            constraint,
        })
    }

    /// 解析元数据表达式（第一行第一个单元格）
    pub fn parse_meta_expr(&mut self) -> ParseResult<MetaExpr> {
        if self.tokens.is_empty() {
            return Err(ParseError::new(ParseErrorKind::EmptyInput, 0));
        }

        // 解析表类型 @dict, @class, @enum, @lang, @config
        self.expect(TokenKind::At)?;
        let kind_token = self.expect(TokenKind::Identifier)?;
        let kind = match kind_token.text.to_lowercase().as_str() {
            "dict" => TableKind::Dict,
            "class" => TableKind::Class,
            "enum" => TableKind::Enum,
            "lang" => TableKind::Lang,
            "config" => TableKind::Config,
            _ => return Err(ParseError::new(ParseErrorKind::InvalidTypeName(kind_token.text.clone()), kind_token.start)),
        };

        // 解析可选的复合唯一约束 @unique(field1, field2)
        let mut unique_fields = Vec::new();
        if self.current().kind == TokenKind::At {
            self.advance();
            let unique_token = self.expect(TokenKind::Identifier)?;
            if unique_token.text.eq_ignore_ascii_case("unique") {
                self.expect(TokenKind::LeftParen)?;
                if self.current().kind != TokenKind::RightParen {
                    let field = self.expect(TokenKind::Identifier)?;
                    unique_fields.push(field.text);
                    while self.current().kind == TokenKind::Comma {
                        self.advance();
                        let field = self.expect(TokenKind::Identifier)?;
                        unique_fields.push(field.text);
                    }
                }
                self.expect(TokenKind::RightParen)?;
            }
        }

        if !self.is_at_end() {
            return Err(ParseError::unexpected_token(
                "输入结束",
                Some(&self.current().text),
                self.current().start,
            ));
        }

        Ok(MetaExpr { kind, unique_fields })
    }

    /// 解析字段元属性（Excel 注释中）
    pub fn parse_field_metas(&mut self) -> ParseResult<Vec<FieldMeta>> {
        if self.tokens.is_empty() {
            return Ok(Vec::new());
        }

        let mut metas = Vec::new();
        while !self.is_at_end() {
            let meta = self.parse_field_meta()?;
            metas.push(meta);
        }

        Ok(metas)
    }

    /// 解析单个字段元属性
    fn parse_field_meta(&mut self) -> ParseResult<FieldMeta> {
        self.expect(TokenKind::At)?;
        let name_token = self.expect(TokenKind::Identifier)?;
        
        match name_token.text.to_lowercase().as_str() {
            "primary" => Ok(FieldMeta::Primary),
            "virtual" => Ok(FieldMeta::Virtual),
            "computed" => Ok(FieldMeta::Computed),
            "default" => {
                self.expect(TokenKind::LeftParen)?;
                let value = self.parse_meta_value()?;
                self.expect(TokenKind::RightParen)?;
                Ok(FieldMeta::Default(value))
            }
            _ => Err(ParseError::new(ParseErrorKind::InvalidTypeName(name_token.text.clone()), name_token.start)),
        }
    }

    /// 解析类型元属性（Excel 注释中）
    pub fn parse_type_metas(&mut self) -> ParseResult<Vec<TypeMeta>> {
        if self.tokens.is_empty() {
            return Ok(Vec::new());
        }

        let mut metas = Vec::new();
        while !self.is_at_end() {
            let meta = self.parse_type_meta()?;
            metas.push(meta);
        }

        Ok(metas)
    }

    /// 解析单个类型元属性
    fn parse_type_meta(&mut self) -> ParseResult<TypeMeta> {
        self.expect(TokenKind::At)?;
        let name_token = self.expect(TokenKind::Identifier)?;
        
        match name_token.text.to_lowercase().as_str() {
            "min" => {
                self.expect(TokenKind::LeftParen)?;
                let value = self.parse_meta_number()?;
                self.expect(TokenKind::RightParen)?;
                Ok(TypeMeta::Min(value))
            }
            "max" => {
                self.expect(TokenKind::LeftParen)?;
                let value = self.parse_meta_number()?;
                self.expect(TokenKind::RightParen)?;
                Ok(TypeMeta::Max(value))
            }
            "range" => {
                self.expect(TokenKind::LeftParen)?;
                let min = self.parse_meta_number()?;
                self.expect(TokenKind::Comma)?;
                let max = self.parse_meta_number()?;
                self.expect(TokenKind::RightParen)?;
                Ok(TypeMeta::Range(min, max))
            }
            "default" => {
                self.expect(TokenKind::LeftParen)?;
                let value = self.parse_meta_value()?;
                self.expect(TokenKind::RightParen)?;
                Ok(TypeMeta::Default(value))
            }
            _ => Err(ParseError::new(ParseErrorKind::InvalidTypeName(name_token.text.clone()), name_token.start)),
        }
    }

    /// 解析元属性值
    fn parse_meta_value(&mut self) -> ParseResult<String> {
        let token = self.current();
        match token.kind {
            TokenKind::Integer | TokenKind::Identifier => {
                self.advance();
                Ok(token.text)
            }
            TokenKind::String => {
                self.advance();
                // 移除引号
                let text = token.text;
                if text.len() >= 2 {
                    Ok(text[1..text.len()-1].to_string())
                } else {
                    Ok(text)
                }
            }
            _ => Err(ParseError::unexpected_token(
                "值",
                Some(&token.kind.to_string()),
                token.start,
            )),
        }
    }

    /// 解析元属性数字
    fn parse_meta_number(&mut self) -> ParseResult<i64> {
        let token = self.expect(TokenKind::Integer)?;
        token.text.parse().map_err(|_| {
            ParseError::new(ParseErrorKind::InvalidNumberLiteral(token.text.clone()), token.start)
        })
    }
}


