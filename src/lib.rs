pub mod data;
pub mod error;
pub mod prelude;

pub use error::{RecolError, RecolResult};

pub fn init() -> RecolResult<()> {
    Ok(())
}

