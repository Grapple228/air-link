// region:    --- Modules

mod error;

use grapple_utils::cuuid::uuid::Uuid;

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: Uuid,
}

// Constructors
impl Ctx {
    const USER_ID_ROOT: Uuid = Uuid::from_u128(0);

    pub fn root_ctx() -> Self {
        Ctx {
            user_id: Self::USER_ID_ROOT,
        }
    }

    pub fn new(user_id: Uuid) -> Result<Self> {
        if user_id == Self::USER_ID_ROOT {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}
