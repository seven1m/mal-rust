#[derive(Debug,PartialEq)]
pub enum MalType {
    List(Vec<MalType>),
    Number(i64),
    Symbol(String),
}
