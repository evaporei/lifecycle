use derive_system::System;
use lifecycle::Lifecycle;

struct App;
impl Lifecycle for App {
    fn start(self) -> Self {
        println!("App::start");
        Self
    }

    fn stop(self) -> Self {
        println!("App::stop");
        Self
    }
}

struct Scheduler;
impl Lifecycle for Scheduler {
    fn start(self) -> Self {
        println!("Scheduler::start");
        Self
    }

    fn stop(self) -> Self {
        println!("Scheduler::stop");
        Self
    }
}

struct Database;
impl Lifecycle for Database {
    fn start(self) -> Self {
        println!("Database::start");
        Self
    }

    fn stop(self) -> Self {
        println!("Database::stop");
        Self
    }
}

#[derive(System)]
pub struct ExampleSystem {
    app: App,
    scheduler: Scheduler,
    database: Database,
}

fn main() {
    let mut system = ExampleSystem {
        app: App,
        scheduler: Scheduler,
        database: Database,
    };

    system = system.start();

    let _ = system.stop();
}
