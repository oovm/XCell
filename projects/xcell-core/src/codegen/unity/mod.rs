use super::*;
use std::fs::File;

impl UnityCodegen {
    pub fn write_xml(&self, table: &XCellTable, f: &mut impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_csharp(&self, table: &XCellTable, f: &mut impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_interface(&self, f: &mut impl Write) -> std::io::Result<usize> {
        let mut slots = HashMap::new();
        slots.insert("NAMESPACE", self.namespace.join("."));
        let render = build_template(include_str!("DefineInterface.cs")).render_nofail(&slots);
        f.write(render.as_bytes())
    }
}

#[test]
fn test() {
    let mut file = File::open("test.cs").unwrap();
    let unity = UnityCodegen { namespace: vec!["Test".to_string()], support_binary: false };
    unity.write_interface(&mut file).unwrap();
}
