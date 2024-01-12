use crate::command_executor::CommandExecutor;
use crate::prelude::*;
use crate::sql_executor::SqlExecutor;
use crate::transaction::Transaction;
use crate::LunaOrmResult;
use std::ops::{Deref, DerefMut};
use tracing::debug;

pub trait Database: CommandExecutor + SqlExecutor + std::fmt::Debug {
    fn get_type(&self) -> &DatabaseType;

    async fn transaction<'a>(&'a self) -> LunaOrmResult<Transaction<'a, Self::G>> {
        let trx = self.get_pool()?.begin().await?;
        let generator = self.get_generator();
        let transaction = Transaction::new(trx, generator);
        return Ok(transaction);
    }

    async fn remove<SE>(
        &mut self,
        primary: &dyn Primary,
        selection: &dyn Selection,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        debug!(target: "luna_orm", command = "remove",  primary = ?primary, selection = ?selection);
        let mut trx = self.get_pool()?.begin().await?;
        let selected_entity: Option<SE> = self.select(primary, selection).await?;
        let sql = self.get_generator().get_delete_sql(primary);
        debug!(target: "luna_orm", command = "remove",  sql = sql);
        let args = primary.any_arguments();
        let result = sqlx::query_with(&sql, args).execute(&mut *trx).await?;
        trx.commit().await?;

        debug!(target: "luna_orm", command = "remove",  result = ?result);
        if result.rows_affected() > 0 {
            return Ok(selected_entity);
        } else {
            return Ok(None);
        }
    }

    async fn transact(&mut self, commands: &[WriteCommand]) -> LunaOrmResult<bool> {
        debug!(target: "luna_orm", command = "transact",  commands = ?commands);
        let trx = self.get_pool()?.begin().await?;
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
        trx.commit().await?;
        debug!(target: "luna_orm", command = "transact",  result = true);
        Ok(true)
    }
}

#[derive(Debug, Clone)]
pub enum DatabaseType {
    SqliteLocal,
    MySql,
    PostgreSql,
}

#[derive(Debug, Clone)]
pub struct DB<T: Database>(pub T);

impl<T> Deref for DB<T>
where
    T: Database,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DB<T>
where
    T: Database,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
