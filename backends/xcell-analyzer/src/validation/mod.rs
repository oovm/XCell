use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::{XError, XResult};

use crate::{WorkspaceManager, x_table::table::XTableReader};
use calamine::Data;
use xcell_core::{XCellTyped, XCellValue};

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
    fn validate(&self, table: &dyn XTableReader, workspace: &WorkspaceManager) -> ValidationResult;
}

/// 引用验证器，用于验证跨表引用的有效性
pub struct RefValidator;

impl RefValidator {
    /// 收集工作区中所有表的主键 ID 集合（字符串形式）
    ///
    /// # 参数
    /// * `workspace` - 工作区管理器
    ///
    /// # 返回值
    /// 返回一个映射，键为表名，值为该表的所有主键 ID 集合（字符串形式）
    fn collect_all_ids(workspace: &WorkspaceManager) -> BTreeMap<String, BTreeSet<String>> {
        let mut all_ids: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for list_data in workspace.lists() {
            let mut ids = BTreeSet::new();
            for id in list_data.mapping.keys() {
                ids.insert(id.to_string());
            }
            all_ids.insert(list_data.name.clone(), ids);
        }

        for dict_data in workspace.dicts() {
            let mut ids = BTreeSet::new();
            for key in dict_data.mapping.keys() {
                ids.insert(key.clone());
            }
            all_ids.insert(dict_data.name.clone(), ids);
        }

        let mut language_ids = BTreeSet::new();
        for key in workspace.languages.store.keys() {
            language_ids.insert(key.clone());
        }
        all_ids.insert("Language".to_string(), language_ids);

        all_ids
    }

    /// 验证单个引用值
    ///
    /// # 参数
    /// * `ref_value` - 引用值（字符串形式）
    /// * `target_table` - 目标表名
    /// * `all_ids` - 所有表的 ID 集合
    /// * `table_name` - 当前表名
    /// * `field_name` - 字段名
    /// * `row` - 行号
    /// * `column` - 列号
    ///
    /// # 返回值
    /// 如果引用无效，返回 Some(XError)，否则返回 None
    pub fn validate_reference(
        ref_value: &str,
        target_table: &str,
        all_ids: &BTreeMap<String, BTreeSet<String>>,
        table_name: &str,
        field_name: &str,
        row: usize,
        column: usize,
    ) -> Option<XError> {
        if ref_value.is_empty() || ref_value == "0" {
            return None;
        }

        match all_ids.get(target_table) {
            Some(ids) => {
                if !ids.contains(ref_value) {
                    return Some(
                        XError::runtime_error(format!(
                            "跨表引用验证失败: 表 '{}' 字段 '{}' 第 {} 行引用了表 '{}' 中不存在的 ID: {}",
                            table_name, field_name, row + 1, target_table, ref_value
                        ))
                        .with_xy(column, row),
                    );
                }
            }
            None => {
                return Some(
                    XError::runtime_error(format!(
                        "跨表引用验证失败: 表 '{}' 字段 '{}' 第 {} 行引用了不存在的目标表: '{}'",
                        table_name, field_name, row + 1, target_table
                    ))
                    .with_xy(column, row),
                );
            }
        }

        None
    }

    /// 验证列表中的引用值
    ///
    /// # 参数
    /// * `values` - 列表中的值
    /// * `target_table` - 目标表名
    /// * `all_ids` - 所有表的 ID 集合
    /// * `table_name` - 当前表名
    /// * `field_name` - 字段名
    /// * `row` - 行号
    /// * `column` - 列号
    ///
    /// # 返回值
    /// 返回所有验证错误的列表
    fn validate_list_references(
        values: &[XCellValue],
        target_table: &str,
        all_ids: &BTreeMap<String, BTreeSet<String>>,
        table_name: &str,
        field_name: &str,
        row: usize,
        column: usize,
    ) -> Vec<XError> {
        let mut errors = Vec::new();

        for value in values {
            if let XCellValue::Reference(ref_value) = value {
                if let Some(error) = Self::validate_reference(
                    ref_value,
                    target_table,
                    all_ids,
                    table_name,
                    field_name,
                    row,
                    column,
                ) {
                    errors.push(error);
                }
            }
        }

        errors
    }
}

impl Validator for RefValidator {
    fn validate(&self, table: &dyn XTableReader, workspace: &WorkspaceManager) -> ValidationResult {
        let mut result = ValidationResult::new();

        let all_ids = Self::collect_all_ids(workspace);

        let table_name = table.get_name();

        let headers: Vec<_> = table.headers().collect();

        for (row, data) in table.rows() {
            if row < 2 {
                continue;
            }

            let first_cell = data.first().unwrap_or(&Data::Empty);
            if matches!(first_cell, Data::Empty) {
                continue;
            }

            for header in &headers {
                let cell = data.get(header.column).unwrap_or(&Data::Empty);

                if matches!(cell, Data::Empty) {
                    continue;
                }

                match &header.typing {
                    XCellTyped::Reference(ref_desc) => {
                        let target_table = ref_desc.target_table.as_str();

                        let xdata = xcell_provider::convert_data(cell);
                        match ref_desc.parse_cell(&xdata) {
                            Ok(XCellValue::Reference(ref_value)) => {
                                if let Some(error) = Self::validate_reference(
                                    &ref_value,
                                    target_table,
                                    &all_ids,
                                    &table_name,
                                    &header.field_name,
                                    row,
                                    header.column,
                                ) {
                                    result.add_error(error);
                                }
                            }
                            Ok(_) => {}
                            Err(e) => {
                                result.add_error(e.with_xy(header.column, row));
                            }
                        }
                    }
                    XCellTyped::List(list_desc) => {
                        if list_desc.element_type.is_reference() {
                            if let Some(ref_desc) = list_desc.element_type.as_reference() {
                                let target_table = ref_desc.target_table.as_str();

                                let xdata = xcell_provider::convert_data(cell);
                                match list_desc.parse_cell(&xdata) {
                                    Ok(XCellValue::Vector(values)) => {
                                        let errors = Self::validate_list_references(
                                            &values,
                                            target_table,
                                            &all_ids,
                                            &table_name,
                                            &header.field_name,
                                            row,
                                            header.column,
                                        );
                                        for error in errors {
                                            result.add_error(error);
                                        }
                                    }
                                    Ok(_) => {}
                                    Err(e) => {
                                        result.add_error(e.with_xy(header.column, row));
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        result
    }
}

/// Path 检查 - 验证资源路径是否有效
pub struct PathValidator;

impl Validator for PathValidator {
    fn validate(&self, table: &dyn XTableReader, workspace: &WorkspaceManager) -> ValidationResult {
        let mut result = ValidationResult::new();
        // TODO: 实现 Path 检查逻辑
        result
    }
}

/// Range 检查 - 验证数值是否在合理范围内
pub struct RangeValidator;

impl Validator for RangeValidator {
    fn validate(&self, table: &dyn XTableReader, workspace: &WorkspaceManager) -> ValidationResult {
        let mut result = ValidationResult::new();
        // TODO: 实现 Range 检查逻辑
        result
    }
}

/// 一致性检查 - 验证跨表格数据的一致性
pub struct ConsistencyValidator;

impl Validator for ConsistencyValidator {
    fn validate(&self, table: &dyn XTableReader, workspace: &WorkspaceManager) -> ValidationResult {
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
    pub fn validate(&self, table: &dyn XTableReader, workspace: &WorkspaceManager) -> ValidationResult {
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
