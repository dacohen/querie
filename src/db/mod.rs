use std::io::Error;

pub mod postgres;

pub enum DBResultKind {
    String,
    Number,
    Boolean,
    Timestamp,
}

pub struct DBResult {
    pub column_name: String,
    pub value: String,
    pub kind: DBResultKind,
}

pub trait Queryable {
    fn query(&mut self, q: String, args: Vec<String>) -> Result<Vec<Vec<DBResult>>, Error>;
}
