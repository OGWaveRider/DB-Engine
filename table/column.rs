#[derive(Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    Bool,
    VarChar(usize),
}

pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub offset: usize,
    pub size: usize,
}