use crate::Lifecycle;
use std::sync::Arc;

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

struct Scheduler;

impl Lifecycle for Scheduler {}

#[derive(Clone)]
pub struct ConfigOptions<'a> {
    host: &'a str,
    port: &'a str,
}

impl<'a> ConfigOptions<'a> {
    pub fn new(host: &'a str, port: &'a str) -> Self {
        Self { host, port }
    }
}

pub struct App<'a> {
    options: ConfigOptions<'a>,
    cache: Arc<()>,
}

impl<'a> App<'a> {
    fn new(options: ConfigOptions<'a>) -> Self {
        Self {
            options,
            cache: Arc::new(()),
        }
    }
}

impl Lifecycle for App<'_> {}

pub struct ExampleSystem<'a> {
    db: Database<'a>,
    scheduler: Scheduler,
    app: App<'a>,
    config_options: ConfigOptions<'a>,
}

impl<'a> ExampleSystem<'a> {
    pub fn new(config_options: ConfigOptions<'a>) -> Self {
        Self {
            db: Database::new(config_options.host, config_options.port),
            scheduler: Scheduler,
            app: App::new(config_options.clone()),
            config_options,
        }
    }
}

impl Lifecycle for ExampleSystem<'_> {
    fn start(self) -> Self {
        let mut system = self;
        system.db = system.db.start();
        system.scheduler = system.scheduler.start();
        system.app = system.app.start();
        system
    }
    fn stop(self) -> Self {
        let mut system = self;
        system.app = system.app.stop();
        system.scheduler = system.scheduler.stop();
        system.db = system.db.stop();
        system
    }
}
