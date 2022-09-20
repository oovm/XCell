
use super::*;

impl CalamineTable {
    pub fn rows(&self) -> CalamineTableRows {
        let mut out = CalamineTableRows { rows: self.table.rows(), this: 0 };
        for _ in 1..self.config.line.data {
            out.next();
        }
        out
    }
}

pub struct CalamineTableRows<'i> {
    rows: Rows<'i, DataType>,
    this: usize,
}

impl<'i> Iterator for CalamineTableRows<'i> {
    type Item = (usize, &'i [DataType]);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.rows.next()?;
        let out = (self.this, item);
        self.this += 1;
        Some(out)
    }
}
