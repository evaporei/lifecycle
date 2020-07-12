pub trait Lifecycle: Sized {
    fn start(self) -> Self {
        self
    }
    fn stop(self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests;
