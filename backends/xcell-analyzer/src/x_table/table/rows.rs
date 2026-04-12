use super::*;

impl CalamineTableAdapter {
    pub fn rows(&self) -> CalamineTableRows<'_> {
        let mut out = CalamineTableRows { rows: self.inner.inner_table().rows(), this: 0 };
        for _ in 1..self.config.line.data {
            out.next();
        }
        out
    }
}

pub struct CalamineTableRows<'i> {
    rows: Rows<'i, Data>,
    this: usize,
}

impl<'i> Iterator for CalamineTableRows<'i> {
    type Item = (usize, &'i [Data]);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.rows.next()?;
        let out = (self.this, item);
        self.this += 1;
        Some(out)
    }
}
