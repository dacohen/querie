use std::io::{self, Error};

use chrono::Utc;
use postgres::{
    types::{ToSql, Type},
    NoTls,
};

use super::Queryable;

pub struct Client {
    postgres_client: postgres::Client,
}

impl Client {
    pub fn new(conn_string: &str) -> Result<Client, Error> {
        match postgres::Client::connect(conn_string, NoTls) {
            Ok(c) => Ok(Client { postgres_client: c }),
            Err(e) => Err(Error::new(io::ErrorKind::ConnectionAborted, e.to_string())),
        }
    }
}

fn col_to_string_kind(row: &postgres::Row, index: usize) -> (String, super::DBResultKind) {
    let column_type = row.columns().get(index).map(|c| c.type_()).unwrap();
    let (value, kind) = match column_type {
        &Type::BOOL => {
            let v: Option<bool> = row.get(index);
            (v.map(|v| v.to_string()), super::DBResultKind::Boolean)
        }
        &Type::TEXT | &Type::VARCHAR | &Type::CHAR_ARRAY | &Type::NAME => {
            let v: Option<String> = row.get(index);
            (v, super::DBResultKind::String)
        }
        &Type::INT2 | &Type::INT4 | &Type::INT8 => {
            let v: Option<i64> = row.get(index);
            (v.map(|v| v.to_string()), super::DBResultKind::Number)
        }
        &Type::FLOAT4 | &Type::FLOAT8 => {
            let v: Option<f64> = row.get(index);
            (v.map(|v| v.to_string()), super::DBResultKind::Number)
        }
        &Type::TIMESTAMP | &Type::TIMESTAMPTZ => {
            let v: Option<chrono::DateTime<Utc>> = row.get(index);
            (v.map(|v| v.to_string()), super::DBResultKind::Timestamp)
        }
        &_ => (Some("PARSE ERROR".to_string()), super::DBResultKind::String),
    };
    (value.unwrap_or("".to_string()), kind)
}

impl Queryable for Client {
    fn query(&mut self, q: String, args: Vec<String>) -> Result<Vec<Vec<super::DBResult>>, Error> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for arg in &args {
            params.push(arg)
        }
        match self.postgres_client.query(&q, &params[..]) {
            Ok(rows) => {
                if rows.len() > 0 {
                    let column_names: Vec<String> = rows[0]
                        .columns()
                        .iter()
                        .map(|c| c.name().to_string())
                        .collect();

                    return Ok(rows
                        .iter()
                        .map(|r| {
                            let mut results: Vec<super::DBResult> = Vec::new();
                            for (idx, col_name) in column_names.iter().enumerate() {
                                let (value, kind) = col_to_string_kind(r, idx);
                                results.push(super::DBResult {
                                    column_name: col_name.clone(),
                                    value,
                                    kind,
                                });
                            }
                            results
                        })
                        .collect());
                }
                Ok(Vec::new())
            }
            Err(e) => Err(Error::new(io::ErrorKind::InvalidData, e.to_string())),
        }
    }
}
