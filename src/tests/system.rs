use crate::tests::mocks::{ConfigOptions, ExampleSystem};
use crate::Lifecycle;

#[test]
fn example_system() {
    let config_options = ConfigOptions::new("localhost", "3000");

    let system = ExampleSystem::new(config_options);

    let system = system.start();

    system.stop();
}
