use alohomora::context::{Context, ContextData};
use alohomora::db::{BBoxConn, BBoxParams, BBoxResult, BBoxStatement, BBoxValue};
use alohomora::policy::Reason;
use mysql::Opts;
use mysql::*;
use mysql::prelude::*;
use crate::login::ContextDataType;

// Struct representing the SQL backend
pub struct MySQLBackend {
    pub handle: BBoxConn,
    _schema: String,
    _db_user: String,
    _db_password: String,
    _db_name: String,
}

impl MySQLBackend {
    pub fn new(
        user: &str, 
        password: &str, 
        dbname: &str, 
        prime: bool,
    ) -> Result<Self> {
        let schema = std::fs::read_to_string("src/schema.sql").unwrap();

        let mut db = BBoxConn::new(
            Opts::from_url(&format!("mysql://{}:{}@127.0.0.1/", user, password)).unwrap()
        ).unwrap();

        assert_eq!(db.ping(), true);

        if prime {
            println!("[!] priming");
            db.query_drop(format!("DROP DATABASE IF EXISTS {};", dbname)).unwrap();
            db.query_drop(format!("CREATE DATABASE {};", dbname)).unwrap();
            db.query_drop(format!("USE {};", dbname)).unwrap();

            for line in schema.lines(){
                if line.starts_with("--") || line.is_empty() { continue };
                db.query_drop(line).unwrap();
            }
        } else {
            db.query_drop(format!("USE {};", dbname)).unwrap();
        }

        Ok(MySQLBackend{
            handle: db,
            _schema: schema.to_owned(),
            _db_user: user.to_string().to_owned(),
            _db_password: password.to_string().to_owned(),
            _db_name: dbname.to_string().to_owned(),
        })
    }

    // Needs to take context
    // pub fn prep_exec<Q, P, T>(&mut self, query: Q, params: P, context: Context<ContextData>) -> Result<Vec<T>>
    // where
    //     Q: AsRef<str>,
    //     P: Into<Params>,
    //     T: FromRow, {
    //         self.handle.prep_exec(query, params, context)
    // }

    pub fn prep_exec<P: Into<BBoxParams>>(&mut self, sql: &str, params: P, context: Context<ContextDataType>) -> Vec<Vec<BBoxValue>> {
        if !self.prep_stmts.contains_key(sql) {
            let stmt = self
                .handle
                .prep(sql)
                .expect(&format!("failed to prepare statement \'{}\'", sql));
            self.prep_stmts.insert(sql.to_owned(), stmt);
        }
        
        let params: BBoxParams = params.into();
        loop {
            match self
                .handle
                .exec_iter(self.prep_stmts[sql].clone(), params.clone(), context.clone())
            {
                Err(e) => {}
                Ok(res) => {
                    let mut rows = vec![];
                    for row in res {
                        rows.push(row.unwrap().unwrap());
                    }
                    //debug!(self.log, "executed query {}, got {} rows", sql, rows.len());
                    return rows;
                }
            }
            self.reconnect();
        }
    }
    // pub fn prep_exec<'i, P: Into<BBoxParams>, D: ContextData, T: FromRow>(
    //     &mut self, query: &str, params: P, context: Context<D>) -> BBoxResult<Vec<T>> {
    //         let stmt = self.prep(query)?;
    //         self.exec(stmt, params, context)
    // }
    // pub fn exec<S: for<'a> Into<BBoxStatement<'a>>, P: Into<BBoxParams>, D: ContextData, T: FromRow>(
    //     &mut self,
    //     stmt: S,
    //     params: P,
    //     context: Context<D>,
    // ) -> BBoxResult<Vec<T>> {
    //     let stmt = stmt.into();
    //     let (statement, stmt_str) = (stmt.0, stmt.1);
    //     let statement = match statement {
    //         Some(statement) => statement,
    //         None => self.conn.prep(stmt_str.deref())?,
    //     };

    //     let params = params.into().transform(context, Reason::DB(stmt_str.deref()))?;
    //     self.conn.exec(statement, params)
    // }
}