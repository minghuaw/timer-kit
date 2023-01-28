#[derive(Debug)]
pub struct Elapsed {
    _sealed: (),
}

impl Elapsed {
    pub(crate) fn new() -> Self {
        Self { _sealed: () }
    }
}

impl std::fmt::Display for Elapsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "timer elapsed")
    }
}

impl std::error::Error for Elapsed {}