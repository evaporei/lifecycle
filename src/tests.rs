use crate::Lifecycle;

struct Database<'a> {
    host: &'a str,
    port: &'a str,
    connection: Option<Connection>,
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
    fn new(host: &'a str, port: &'a str) -> Self {
        Self {
            host,
            port,
            connection: None,
        }
    }
    fn get_user(&self, username: &str) -> QueryResult {
        match &self.connection {
            Some(conn) => conn.execute_query(&["SELECT * FROM users WHERE username = ?", username]),
            None => QueryResult::Error,
        }
    }
    fn add_user(&self, username: &str, favorite_color: &str) -> QueryResult {
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

struct Connection;

impl Connection {
    fn execute_query(&self, _args: &[&str]) -> QueryResult {
        println!("execute_query");

        QueryResult::Search
    }
    fn execute_insert(&self, _args: &[&str]) -> QueryResult {
        println!("execute_insert");

        QueryResult::Insert
    }
    fn close(&self) {
        println!("Closing database connection");
    }
}

#[derive(Debug, PartialEq)]
enum QueryResult {
    Search,
    Insert,
    Error,
}

fn connect_to_database(_host: &str, _port: &str) -> Connection {
    println!("Opening database connection");

    Connection
}

#[test]
fn simple_database() {
    let database = Database::new("localhost", "3000");
    assert!(database.connection.is_none());

    let database = database.start();
    assert!(database.connection.is_some());

    let add_user_result = database.add_user("joana", "green");
    assert_eq!(add_user_result, QueryResult::Insert);

    let get_user_result = database.get_user("joana");
    assert_eq!(get_user_result, QueryResult::Search);

    let database = database.stop();
    assert!(database.connection.is_none());
}
