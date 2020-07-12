# component

Rust implementation of https://github.com/stuartsierra/component.

## To Do

- [x] Lifecycle trait
- [ ] System functions and/or macros

While Systems are not implemented in this library, they can be created manually by users, they just have to define correctly when each component should start/stop. There is an example on `src/tests/system.rs` that shows how to do it for now.
