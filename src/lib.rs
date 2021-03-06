pub mod cli;
pub mod fio;
pub mod task;

pub trait Serializer {
    type Type;

    fn serialize(self) -> String;
    fn deserialize(src: &str) -> Option<Self::Type>;
}
