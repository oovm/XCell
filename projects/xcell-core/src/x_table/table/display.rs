use super::*;

impl Default for XCellTable {
    fn default() -> Self {
        Self {
            //
            path: Default::default(),
            headers: Default::default(),
            data: Array2D::filled_with(XCellValue::Boolean(false), 1, 1),
            config: Default::default(),
            sum_excel: 0,
            sum_config: 0,
        }
    }
}

impl Display for XCellTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl PartialEq<Self> for XCellTable {
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(&other.id())
    }
}

impl Eq for XCellTable {}

impl Hash for XCellTable {
    /// 是否要重新加载只取决于表格和配置是否发生变化
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.sum_excel);
        state.write_u64(self.sum_config);
    }
}
