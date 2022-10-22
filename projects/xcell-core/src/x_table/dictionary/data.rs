use std::str::FromStr;

use xcell_errors::for_3rd::FromPrimitive;
use xcell_types::IntegerKind;

use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XListData {
    pub name: String,
    pub id_type: IntegerKind,
    pub headers: Vec<XCellHeader>,
    pub mapping: BTreeMap<BigInt, XDataLine>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDictData {
    pub name: String,
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
    /// 该表单数据的注释
    pub comment: XDocument,
    /// 该表单数据的有效值
    pub data: Vec<XCellValue>,
}

impl XDataLine {
    pub fn parse_key_cell(data: &[DataType], headers: &[XCellHeader], errors: &mut Vec<XError>) -> XResult<Self> {
        let mut out = Self::default();
        out.key = out.check_parse_key(data)?;
        out.try_parse_data(data, headers, errors);
        Ok(out)
    }
    pub fn parse_id_cell(data: &[DataType], headers: &[XCellHeader], errors: &mut Vec<XError>) -> XResult<Self> {
        let mut out = Self::default();
        out.id = out.check_parse_id(data)?;
        out.try_parse_data(data, headers, errors);
        Ok(out)
    }
    fn check_parse_key(&self, data: &[DataType]) -> XResult<String> {
        match data.get(0).and_then(|s| s.get_string()) {
            Some(s) => Ok(s.to_string()),
            None => Err(XError::runtime_error("key 不能为空").with_x(0))?,
        }
    }
    fn check_parse_id(&self, data: &[DataType]) -> XResult<BigInt> {
        match data.get(0) {
            Some(cell) => match cell {
                DataType::String(str) => Ok(BigInt::from_str(str)?),
                DataType::Int(int) => Ok(BigInt::from(*int)),
                DataType::Float(float) => match (float.round() - float) < 0.01f64 {
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
    fn try_parse_data(&mut self, data: &[DataType], headers: &[XCellHeader], errors: &mut Vec<XError>) {
        for (column, (data, header)) in data.iter().zip(headers.iter()).enumerate() {
            match header.typing.parse_cell(data) {
                Ok(o) => self.data.push(o),
                Err(e) => errors.push(e.with_x(column)),
            }
        }
    }
}
