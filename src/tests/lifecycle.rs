use crate::tests::mocks::{Database, QueryResult};
use crate::Lifecycle;

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
