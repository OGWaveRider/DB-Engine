use crate::column::Column;

pub struct TableSchema {
    pub table_name: String,
    pub columns: Vec<Column>,
    pub row_size: usize,
}