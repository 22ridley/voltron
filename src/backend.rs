use mysql::Opts;
use mysql::*;
use mysql::prelude::*;

// Struct representing the SQL backend
pub struct MySQLBackend {
    pub handle: mysql::Conn,
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

        let mut db = mysql::Conn::new(
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
}