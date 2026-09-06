use std::collections::HashMap;
use crate::table_schema::TableSchema;
use crate::column::{Column, DataType};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    Text(String),
}

pub struct RowCodec<'a> {
    pub schema: &'a TableSchema,
}

impl<'a> RowCodec<'a> {
    pub fn encode(&self, row: &HashMap<String, Value>) -> Vec<u8> {
        let mut buffer = vec![0u8; self.schema.row_size];

        for col in &self.schema.columns {
            if let Some(value) = row.get(&col.name) {
                self.write_at_offset(&mut buffer, col, value);
            }
        }

        buffer
    }

    pub fn decode(&self, raw_bytes: &[u8]) -> HashMap<String, Value> {
        let mut row = HashMap::new();

        for col in &self.schema.columns {
            let value = self.read_at_offset(raw_bytes, col);
            row.insert(col.name.clone(), value);
        }

        row
    }

    fn write_at_offset(&self, buffer: &mut [u8], col: &Column, value: &Value) {
        let start = col.offset;
        let end = start + col.size;

        match (value, &col.data_type) {
            (Value::Int(v), DataType::Int) => {
                buffer[start..end].copy_from_slice(&v.to_le_bytes());
            }
            (Value::Float(v), DataType::Float) => {
                buffer[start..end].copy_from_slice(&v.to_le_bytes());
            }
            (Value::Bool(v), DataType::Bool) => {
                buffer[start] = *v as u8;
            }
            (Value::Text(v), DataType::VarChar(max_size)) => {
                let bytes = v.as_bytes();
                let len = bytes.len().min(*max_size);
                buffer[start..start + len].copy_from_slice(&bytes[..len]);
            }
            _ => panic!("type mismatch encoding column {}", col.name),
        }
    }

    fn read_at_offset(&self, buffer: &[u8], col: &Column) -> Value {
        let start = col.offset;
        let end = start + col.size;
        let slice = &buffer[start..end];

        match col.data_type {
            DataType::Int => Value::Int(i32::from_le_bytes(slice.try_into().unwrap())),
            DataType::Float => Value::Float(f32::from_le_bytes(slice.try_into().unwrap())),
            DataType::Bool => Value::Bool(slice[0] != 0),
            DataType::VarChar(_) => {
                let end_of_str = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
                Value::Text(String::from_utf8_lossy(&slice[..end_of_str]).into_owned())
            }
        }
    }
}