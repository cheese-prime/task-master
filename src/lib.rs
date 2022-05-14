use std::error::Error;

pub mod cli;
pub mod task;

pub trait Serializer {
    type Type;

    fn serialize(self) -> String;
    fn deserialize(src: &str) -> Option<Self::Type>;
}

pub trait ToTableLikeString {
    fn to_table_string(&self, width: usize) -> String;
}
