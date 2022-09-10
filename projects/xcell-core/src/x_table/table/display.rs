use super::*;

impl Display for XTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl PartialEq<Self> for XTable {
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(&other.id())
    }
}

impl Eq for XTable {}

impl Hash for XTable {
    /// 是否要重新加载只取决于表格和配置是否发生变化
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.sum_excel);
        state.write_u64(self.sum_config);
    }
}
