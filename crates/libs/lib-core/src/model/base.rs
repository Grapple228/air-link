use grapple_utils::cuuid::uuid::Uuid;

use crate::ctx::Ctx;
use crate::model::{Error, Result};

use super::ModelManager;

const LIST_LIMIT_DEFAULT: i64 = 1000;
const LIST_LIMIT_MAX: i64 = 5000;

// CRUD operations over DB

pub struct TableRef;

pub trait DbBmc {
    const TABLE: &'static str;

    fn table_ref() -> TableRef {
        TableRef
    }
}

pub async fn create<MC, E>(ctx: &Ctx, mm: &ModelManager, data: E) -> Result<Uuid> {
    let _db = mm.db();

    todo!()
}

pub async fn create_many<MC, E>(ctx: &Ctx, mm: &ModelManager, data: Vec<E>) -> Result<Vec<Uuid>>
where
    MC: DbBmc,
{
    if data.is_empty() {
        return Ok(vec![]);
    }

    let db = mm.db();

    todo!()
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<E>
where
    MC: DbBmc,
{
    let db = mm.db();

    todo!()
}

pub async fn list<MC, E, F, O>(
    _ctx: &Ctx,
    mm: &ModelManager,
    filters: Option<F>,
    list_options: Option<O>,
) -> Result<Vec<E>>
where
    MC: DbBmc,
{
    let db = mm.db();

    todo!()
}

pub async fn update<MC, E>(ctx: &Ctx, mm: &ModelManager, id: Uuid, data: E) -> Result<()>
where
    MC: DbBmc,
{
    let db = mm.db();

    let count = 0;

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<()>
where
    MC: DbBmc,
{
    let db = mm.db();

    let count = 0;

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}
