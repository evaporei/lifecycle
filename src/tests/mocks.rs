use crate::Lifecycle;

pub struct Database<'a> {
    host: &'a str,
    port: &'a str,
    pub connection: Option<Connection>,
}

impl Lifecycle for Database<'_> {
    fn start(self) -> Self {
        println!("Starting database");

        let conn = connect_to_database(self.host, self.port);

        Database {
            connection: Some(conn),
            ..self
        }
    }
    fn stop(self) -> Self {
        println!("Stopping database");

        self.connection.map(|conn| conn.close());

        Database {
            connection: None,
            ..self
        }
    }
}

impl<'a> Database<'a> {
    pub fn new(host: &'a str, port: &'a str) -> Self {
        Self {
            host,
            port,
            connection: None,
        }
    }
    pub fn get_user(&self, username: &str) -> QueryResult {
        match &self.connection {
            Some(conn) => conn.execute_query(&["SELECT * FROM users WHERE username = ?", username]),
            None => QueryResult::Error,
        }
    }
    pub fn add_user(&self, username: &str, favorite_color: &str) -> QueryResult {
        match &self.connection {
            Some(conn) => conn.execute_insert(&[
                "INSERT INTO users (username, favorite_color)",
                username,
                favorite_color,
            ]),
            None => QueryResult::Error,
        }
    }
}

pub struct Connection;

pub fn connect_to_database(_host: &str, _port: &str) -> Connection {
    println!("Opening database connection");

    Connection
}

impl Connection {
    pub fn execute_query(&self, _args: &[&str]) -> QueryResult {
        println!("execute_query");

        QueryResult::Search
    }
    pub fn execute_insert(&self, _args: &[&str]) -> QueryResult {
        println!("execute_insert");

        QueryResult::Insert
    }
    pub fn close(&self) {
        println!("Closing database connection");
    }
}

#[derive(Debug, PartialEq)]
pub enum QueryResult {
    Search,
    Insert,
    Error,
}
