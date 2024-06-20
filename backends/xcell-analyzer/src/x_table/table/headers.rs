use super::*;

pub struct CalamineTableHeaders<'i> {
    table: &'i CalamineTable,
    this: usize,
    last: usize,
}

impl CalamineTable {
    pub fn headers(&self) -> CalamineTableHeaders<'_> {
        let max_width = self.config.fields.len();
        CalamineTableHeaders { table: self, this: 0, last: max_width }
    }
}

impl<'i> Iterator for CalamineTableHeaders<'i> {
    type Item = XCellHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.this > self.last {
            return None;
        }
        let out = self.table.get_header(self.this);
        self.this += 1;
        Some(out)
    }
}
