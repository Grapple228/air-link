use derive_more::derive::From;
use grapple_utils::cuuid::uuid::Uuid;

use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    EntityNotFound {
        entity: &'static str,
        id: Uuid,
    },
    ListLimitOverMax {
        max: i64,
        actual: i64,
    },
    CreateLimitOverMax {
        max: i64,
        actual: i64,
    },
    // -- Modules
    #[from]
    Store(store::Error),
    // -- Externals
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
