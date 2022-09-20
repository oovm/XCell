use super::*;


pub struct CalamineTableHeaders<'i> {
    table: &'i CalamineTable,
    this: usize,
    last: usize,
}

impl CalamineTable {
    pub fn headers(&self) -> CalamineTableHeaders {
        let mut max_width = 0;
        for i in self.table.rows() {
            let width = i.len();
            if width > max_width {
                max_width = width;
            }
        }
        CalamineTableHeaders {
            table: &self,
            this: 0,
            last: max_width,
        }
    }
}

impl<'i> Iterator for CalamineTableHeaders<'i> {
    type Item = XCellHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.this >= self.last {
            return None;
        }
        self.this += 1;
        Some(self.table.get_header(self.this))
    }
}
