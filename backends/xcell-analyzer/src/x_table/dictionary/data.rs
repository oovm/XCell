use calamine::{Data, DataType};
use std::path::PathBuf;
use std::str::FromStr;

use crate::utils::comment::XComment;
use crate::XCellHeader;
use xcell_core::{
    IntegerKind, XCellValue, XError, XResult,
    for_3rd::{BigInt, FromPrimitive},
};

use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XListData {
    pub name: String,
    pub path: PathBuf,
    pub id_type: IntegerKind,
    pub headers: Vec<XCellHeader>,
    pub mapping: BTreeMap<BigInt, XDataLine>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDictData {
    pub name: String,
    pub path: PathBuf,
    pub headers: Vec<XCellHeader>,
    pub mapping: BTreeMap<String, XDataLine>,
}

/// 表单中的一行数据
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDataLine {
    /// 该表单数据的编号
    pub id: BigInt,
    /// 该表单数据的键
    pub key: String,
    /// 该表单数据的行号
    pub row: usize,
    /// 该表单数据的注释
    pub comment: XComment,
    /// 该表单数据的有效值
    pub data: Vec<XCellValue>,
}

impl XDataLine {
    pub fn parse_key_cell(data: &[Data], row: usize, headers: &[XCellHeader], errors: &mut Vec<XError>) -> XResult<Self> {
        let mut out = Self::default();
        out.row = row;
        out.key = out.check_parse_key(data)?;
        out.try_parse_data(data, row, headers, errors);
        Ok(out)
    }
    pub fn parse_id_cell(data: &[Data], row: usize, headers: &[XCellHeader], errors: &mut Vec<XError>) -> XResult<Self> {
        let mut out = Self::default();
        out.row = row;
        out.id = out.check_parse_id(data)?;
        out.try_parse_data(data, row, headers, errors);
        Ok(out)
    }
    fn check_parse_key(&self, data: &[Data]) -> XResult<String> {
        match data.get(0) {
            Some(cell) => match cell {
                Data::String(str) => Ok(str.to_string()),
                Data::Int(int) => Ok(int.to_string()),
                Data::Float(float) => Ok(float.to_string()),
                _ => Err(XError::runtime_error(format!("key 必须是字符串或数字类型, 实际为 {:?}", cell)).with_x(0))?,
            },
            None => Err(XError::runtime_error("key 不能为空").with_x(0))?,
        }
    }
    fn check_parse_id(&self, data: &[Data]) -> XResult<BigInt> {
        match data.get(0) {
            Some(cell) => match cell {
                Data::String(str) => Ok(BigInt::from_str(str)?),
                Data::Int(int) => Ok(BigInt::from(*int)),
                Data::Float(float) => match (float.round() - float) < 0.01f64 {
                    true => Ok(BigInt::from_f64(*float).unwrap_or_default()),
                    false => Err(XError::runtime_error(format!("id 必须是整数, 实际为 {float}")))?,
                },
                _ => Err(XError::runtime_error(format!("id 必须是整数类型, 实际为 {cell:?}")))?,
            },
            None => Err(XError::runtime_error("id 不能为空").with_x(0))?,
        }
    }
}

impl XDataLine {
    fn try_parse_data(&mut self, data: &[Data], row: usize, headers: &[XCellHeader], errors: &mut Vec<XError>) {
        for header in headers {
            if let xcell_core::XCellTyped::Unknown = header.typing {
                continue;
            }

            let data = data.get(header.column).unwrap_or(&Data::Empty);
            let xdata = xcell_provider::convert_data(data);
            match header.typing.parse_cell(&xdata) {
                Ok(o) => self.data.push(o),
                Err(e) => errors.push(e.with_xy(header.column, row)),
            }
        }
    }
}
