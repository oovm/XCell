use std::str::FromStr;

use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XListData {
    pub name: String,
    pub map: BTreeMap<BigInt, XDataLine>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDictData {
    pub name: String,
    pub map: BTreeMap<String, XDataLine>,
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
    pub fn parse_key_cell(data: &[DataType], errors: &mut Vec<XError>) -> XResult<Self> {
        let mut out = Self::default();
        out.key = out.try_parse_key(data)?;
        for (column, datum) in data.iter().enumerate().skip(1) {
            todo!()
        }
        Ok(out)
    }

    fn try_parse_key(&self, data: &[DataType]) -> XResult<String> {
        match data.get(0).and_then(|s| s.get_string()) {
            Some(s) => Ok(s.to_string()),
            None => Err(XError::runtime_error("key 不能为空").with_x(0))?,
        }
    }
}

impl XDataLine {
    pub fn parse_id_cell(data: &[DataType], errors: &mut Vec<XError>) -> XResult<Self> {
        let mut out = Self::default();
        out.id = out.try_parse_id(data)?;

        for (column, datum) in data.iter().enumerate().skip(1) {
            todo!()
        }
        Ok(out)
    }

    fn try_parse_id(&self, data: &[DataType]) -> XResult<BigInt> {
        match data.get(0) {
            Some(s) => match s {
                DataType::Int(s) => Ok(BigInt::from(*s)),
                DataType::String(s) => Ok(BigInt::from_str(s)?),
                _ => Err(XError::runtime_error("id 必须是整数"))?,
            },
            None => Err(XError::runtime_error("id 不能为空").with_x(0))?,
        }
    }
}
