use crate::context::ContextDataType;
use alohomora::context::Context;
use alohomora::db::{BBoxConn, BBoxOpts, BBoxParams, BBoxStatement, BBoxValue};
use slog::o;
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;

pub struct MySqlBackend {
    pub handle: BBoxConn,
    pub log: slog::Logger,
    _schema: String,
    prep_stmts: HashMap<String, BBoxStatement>,
}

impl MySqlBackend {
    pub fn new(
        user: &str,
        password: &str,
        dbname: &str,
        log: Option<slog::Logger>,
        prime: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let log = match log {
            None => slog::Logger::root(slog::Discard, o!()),
            Some(l) => l,
        };

        let schema = std::fs::read_to_string("src/schema.sql")?;

        let mut db = BBoxConn::new(
            // this is the user and password from the config.toml file
            BBoxOpts::from_url(&format!("mysql://{}:{}@127.0.0.1/", user, password)).unwrap(),
        )
        .unwrap();
        assert_eq!(db.ping(), true);

        if prime {
            db.query_drop(format!("DROP DATABASE IF EXISTS {};", dbname))
                .unwrap();
            db.query_drop(format!("CREATE DATABASE {};", dbname))
                .unwrap();
            db.query_drop(format!("USE {};", dbname)).unwrap();
            for line in schema.lines() {
                if line.starts_with("--") || line.is_empty() {
                    continue;
                }
                db.query_drop(line).unwrap();
            }
        } else {
            db.query_drop(format!("USE {};", dbname)).unwrap();
        }

        Ok(MySqlBackend {
            handle: db,
            log: log,
            _schema: schema.to_owned(),
            prep_stmts: HashMap::new(),
        })
    }

    pub fn prep_exec<P: Into<BBoxParams>>(
        &mut self,
        sql: &str,
        params: P,
        context: Context<ContextDataType>,
    ) -> Vec<Vec<BBoxValue>> {
        if !self.prep_stmts.contains_key(sql) {
            let stmt = self
                .handle
                .prep(sql)
                .expect(&format!("failed to prepare statement \'{}\'", sql));
            self.prep_stmts.insert(sql.to_owned(), stmt);
        }

        let params: BBoxParams = params.into();
        loop {
            match self.handle.exec_iter(
                self.prep_stmts[sql].clone(),
                params.clone(),
                context.clone(),
            ) {
                Err(_e) => {
                    panic!()
                }
                Ok(res) => {
                    let mut rows = vec![];
                    for row in res {
                        rows.push(row.unwrap().unwrap());
                    }
                    //debug!(self.log, "executed query {}, got {} rows", sql, rows.len());
                    return rows;
                }
            }
        }
    }
}
