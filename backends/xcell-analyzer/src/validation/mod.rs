use std::path::Path;

use crate::{XError, XResult};

use crate::{WorkspaceManager, x_table::table::TableReader};

/// 验证结果
pub struct ValidationResult {
    /// 错误信息
    pub errors: Vec<XError>,
}

impl ValidationResult {
    /// 创建新的验证结果
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// 添加错误
    pub fn add_error(&mut self, error: XError) {
        self.errors.push(error);
    }

    /// 检查是否有错误
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// 合并验证结果
    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
    }
}

/// 验证器 trait
pub trait Validator {
    /// 验证表格
    fn validate(&self, table: &dyn TableReader, workspace: &WorkspaceManager) -> ValidationResult;
}

/// Ref 检查 - 验证配置中的 ID 引用是否存在
pub struct RefValidator;

impl Validator for RefValidator {
    fn validate(&self, table: &dyn TableReader, workspace: &WorkspaceManager) -> ValidationResult {
        let mut result = ValidationResult::new();
        // TODO: 实现 Ref 检查逻辑
        result
    }
}

/// Path 检查 - 验证资源路径是否有效
pub struct PathValidator;

impl Validator for PathValidator {
    fn validate(&self, table: &dyn TableReader, workspace: &WorkspaceManager) -> ValidationResult {
        let mut result = ValidationResult::new();
        // TODO: 实现 Path 检查逻辑
        result
    }
}

/// Range 检查 - 验证数值是否在合理范围内
pub struct RangeValidator;

impl Validator for RangeValidator {
    fn validate(&self, table: &dyn TableReader, workspace: &WorkspaceManager) -> ValidationResult {
        let mut result = ValidationResult::new();
        // TODO: 实现 Range 检查逻辑
        result
    }
}

/// 一致性检查 - 验证跨表格数据的一致性
pub struct ConsistencyValidator;

impl Validator for ConsistencyValidator {
    fn validate(&self, table: &dyn TableReader, workspace: &WorkspaceManager) -> ValidationResult {
        let mut result = ValidationResult::new();
        // TODO: 实现一致性检查逻辑
        result
    }
}

/// 验证管理器
pub struct ValidationManager {
    validators: Vec<Box<dyn Validator>>,
}

impl ValidationManager {
    /// 创建新的验证管理器
    pub fn new() -> Self {
        let validators: Vec<Box<dyn Validator>> =
            vec![Box::new(RefValidator), Box::new(PathValidator), Box::new(RangeValidator), Box::new(ConsistencyValidator)];
        Self { validators }
    }

    /// 验证表格
    pub fn validate(&self, table: &dyn TableReader, workspace: &WorkspaceManager) -> ValidationResult {
        let mut result = ValidationResult::new();
        for validator in &self.validators {
            let validator_result = validator.validate(table, workspace);
            result.merge(validator_result);
        }
        result
    }
}

impl Default for ValidationManager {
    fn default() -> Self {
        Self::new()
    }
}
