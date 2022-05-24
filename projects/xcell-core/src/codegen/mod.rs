#[allow(unused_variables)]
mod csv;
#[allow(unused_variables)]
mod unity;

pub struct UnityCodegen {
    pub namespace: Vec<String>,
    pub support_binary: bool,
}

pub struct CsvCodegen {}

pub struct BinaryCodegen {}
