use crate::error::LunaOrmError;
use crate::sql_generator::SqlGenerator;
use crate::LunaOrmResult;
use sqlx::any::AnyArguments;
use sqlx::any::AnyQueryResult;
use sqlx::any::AnyRow;

use crate::command_executor::CommandExecutor;
use crate::sql_executor::SqlExecutor;
//use async_trait::async_trait;
use luna_orm_trait::{
    Entity, Location, Mutation, Primary, SelectedEntity, Selection, WriteCommand,
};

#[derive(Debug)]
pub struct Transaction<'a, G>
where
    G: SqlGenerator + Sync + std::fmt::Debug,
{
    transaction: sqlx::Transaction<'a, sqlx::Any>,
    sql_generator: &'a G,
}

/*
impl<'a, G> std::fmt::Debug for Transaction<'a, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.transaction.fmt()
    }
}
*/

impl<'a, G> CommandExecutor for Transaction<'a, G>
where
    G: SqlGenerator + Sync + std::fmt::Debug,
{
    type G = G;
    fn get_generator(&self) -> &Self::G {
        self.sql_generator
    }
}

impl<'a, G> SqlExecutor for Transaction<'a, G>
where
    G: SqlGenerator + Sync + std::fmt::Debug,
{
    async fn fetch_optional<SE>(
        &mut self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_opt: Option<SE> = query.fetch_optional(&mut *self.transaction).await?;
        Ok(result_opt)
    }

    async fn fetch_all<SE>(&mut self, stmt: &str, args: AnyArguments<'_>) -> LunaOrmResult<Vec<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_vec: Vec<SE> = query.fetch_all(&mut *self.transaction).await?;
        Ok(result_vec)
    }
    async fn execute(
        &mut self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> LunaOrmResult<AnyQueryResult> {
        Ok(sqlx::query_with(stmt, args)
            .execute(&mut *self.transaction)
            .await?)
    }

    async fn execute_plain(&mut self, stmt: &str) -> LunaOrmResult<AnyQueryResult> {
        Ok(sqlx::query(stmt).execute(&mut *self.transaction).await?)
    }
}

impl<'a, G> Transaction<'a, G>
where
    G: SqlGenerator + Sync + std::fmt::Debug,
{
    pub fn new(trx: sqlx::Transaction<'a, sqlx::Any>, sql_generator: &'a G) -> Self {
        Self {
            transaction: trx,
            sql_generator,
        }
    }

    #[inline]
    pub async fn commit(self) -> Result<(), LunaOrmError> {
        Ok(self.transaction.commit().await?)
    }

    #[inline]
    pub async fn rollback(self) -> Result<(), LunaOrmError> {
        Ok(self.transaction.rollback().await?)
    }

    pub async fn query(&mut self, sql: &str) -> Result<usize, LunaOrmError> {
        let result = sqlx::query(sql).execute(&mut *self.transaction).await?;
        Ok(result.rows_affected() as usize)
    }

    pub async fn remove<SE>(
        &mut self,
        primary: &dyn Primary,
        selection: &dyn Selection,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let selected_entity: Option<SE> = self.select(primary, selection).await?;
        let sql = self.get_generator().get_delete_sql(primary);
        let args = primary.any_arguments();
        let result = sqlx::query_with(&sql, args)
            .execute(&mut *self.transaction)
            .await?;

        if result.rows_affected() > 0 {
            Ok(selected_entity)
        } else {
            Ok(None)
        }
    }

    async fn transact(&mut self, commands: &[WriteCommand]) -> LunaOrmResult<bool> {
        for command in commands {
            match command {
                WriteCommand::Insert { entity } => {
                    self.insert(entity.as_ref()).await?;
                }
                WriteCommand::Upsert { entity } => {
                    self.upsert(entity.as_ref()).await?;
                }
                WriteCommand::Update { mutation, primary } => {
                    self.update(mutation.as_ref(), primary.as_ref()).await?;
                }
                WriteCommand::Change { mutation, location } => {
                    self.change(mutation.as_ref(), location.as_ref()).await?;
                }
                WriteCommand::Delete { primary } => {
                    self.delete(primary.as_ref()).await?;
                }
                WriteCommand::Purify { location } => {
                    self.purify(location.as_ref()).await?;
                }
            }
        }
        Ok(true)
    }
}
