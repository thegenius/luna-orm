use crate::error::LunaOrmError;
use crate::mapper::{GenericDaoMapper, GenericDaoMapperImpl};

use luna_orm_trait::SqlxError;
use luna_orm_trait::{Entity, Location, Mutation, PagedList, Primary, SelectedEntity, Selection};

pub struct Transaction<'a> {
    transaction: sqlx::Transaction<'a, sqlx::Any>,
}

impl<'a> Transaction<'a> {
    pub fn new(trx: sqlx::Transaction<'a, sqlx::Any>) -> Self {
        Self { transaction: trx }
    }

    #[inline]
    pub async fn commit(self) -> Result<(), SqlxError> {
        return Ok(self.transaction.commit().await?);
    }

    #[inline]
    pub async fn rollback(self) -> Result<(), SqlxError> {
        return Ok(self.transaction.rollback().await?);
    }

    pub async fn query(&mut self, sql: &str) -> Result<usize, LunaOrmError> {
        let result = sqlx::query(sql).execute(&mut *self.transaction).await?;
        return Ok(result.rows_affected() as usize);
    }

    #[inline]
    pub async fn select<'e, P, S, SE>(
        &mut self,
        primary: P,
        selection: S,
    ) -> Result<Option<SE>, SqlxError>
    where
        P: Primary + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: Option<SE> = <GenericDaoMapperImpl as GenericDaoMapper>::select(
            &mut *self.transaction,
            primary,
            selection,
        )
        .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn create<'e, E>(&mut self, entity: E) -> Result<E, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::create(&mut *self.transaction, entity)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn insert<'e, E>(&mut self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::insert(&mut *self.transaction, entity)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn upsert<'e, E>(&mut self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::upsert(&mut *self.transaction, entity)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn update<'e, E>(&mut self, entity: E) -> Result<bool, LunaOrmError>
    where
        E: Entity + Send + Clone,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::update(&mut *self.transaction, entity)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn remove<'e, P, E>(&mut self, primary: P) -> Result<E, LunaOrmError>
    where
        P: Primary + Send,
        E: Entity + Send + Clone,
    {
        let result: E =
            <GenericDaoMapperImpl as GenericDaoMapper>::remove(&mut *self.transaction, primary)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn delete<'e, P>(&mut self, primary: P) -> Result<bool, LunaOrmError>
    where
        P: Primary + Send,
    {
        let result: bool =
            <GenericDaoMapperImpl as GenericDaoMapper>::delete(&mut *self.transaction, primary)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn search<'e, EX, L, S, SE>(
        &mut self,
        location: L,
        selection: S,
    ) -> Result<Vec<SE>, LunaOrmError>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: Vec<SE> = <GenericDaoMapperImpl as GenericDaoMapper>::search(
            &mut *self.transaction,
            location,
            selection,
        )
        .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn search_paged<'e, EX, L, S, SE>(
        &mut self,
        location: L,
        selection: S,
    ) -> Result<PagedList<SE>, LunaOrmError>
    where
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let result: PagedList<SE> = <GenericDaoMapperImpl as GenericDaoMapper>::search_paged(
            &mut *self.transaction,
            location,
            selection,
        )
        .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn purify<'e, EX, L>(&mut self, location: L) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
    {
        let result: usize =
            <GenericDaoMapperImpl as GenericDaoMapper>::purify(&mut *self.transaction, location)
                .await?;
        return Ok(result);
    }

    #[inline]
    pub async fn change<'e, EX, L, M>(
        &mut self,
        location: L,
        mutation: M,
    ) -> Result<usize, LunaOrmError>
    where
        L: Location + Send,
        M: Mutation + Send,
    {
        let result: usize = <GenericDaoMapperImpl as GenericDaoMapper>::change(
            &mut *self.transaction,
            location,
            mutation,
        )
        .await?;
        return Ok(result);
    }
}
