use derive_system::System;
use lifecycle::Lifecycle;

struct App;
impl Lifecycle for App {}

struct Scheduler;
impl Lifecycle for Scheduler {}

struct Database;
impl Lifecycle for Database {}

#[derive(System)]
pub struct ExampleSystem {
    _app: App,
    _scheduler: Scheduler,
    _database: Database,
}

fn main() {
}
