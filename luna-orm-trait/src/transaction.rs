use crate::merge_any_arguments;
use crate::v2::GenericDaoMapper;
use crate::SqlxError;
use crate::{Entity, Location, Mutation, PageInfo, PagedList, Primary, SelectedEntity, Selection};
use async_trait::async_trait;
use sqlx::Any;
use sqlx::AnyPool;
use sqlx::Executor;
use sqlx::TransactionManager;
use sqlx::{any::AnyRow, Row};

pub struct Transaction<'a> {
    transaction: sqlx::Transaction<'a, sqlx::Any>,
}

impl<'a> GenericDaoMapper for Transaction<'a> {}

impl<'a> Transaction<'a> {
    #[inline]
    pub async fn commit(self) -> Result<(), SqlxError> {
        return Ok(self.transaction.commit().await?);
    }

    #[inline]
    pub async fn rollback(self) -> Result<(), SqlxError> {
        return Ok(self.transaction.rollback().await?);
    }

    pub async fn select<'e, EX, P, S, SE>(
        &mut self,
        primary: P,
        selection: S,
    ) -> Result<Option<SE>, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        P: Primary + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: Option<SE> =
            <Transaction as GenericDaoMapper>::select(&mut *self.transaction, primary, selection)
                .await?;
        return Ok(result);
    }
}
