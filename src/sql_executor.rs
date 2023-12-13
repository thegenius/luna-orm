use crate::error::LunaOrmError;
use crate::LunaOrmResult;

use async_trait::async_trait;
use luna_orm_trait::*;

use sqlx::any::AnyArguments;
use sqlx::any::AnyQueryResult;
use sqlx::any::AnyRow;
use sqlx::AnyPool;

#[async_trait]
pub trait SqlExecutor {
    fn get_pool(&self) -> LunaOrmResult<&AnyPool> {
        Err(LunaOrmError::NotImplement)
    }

    async fn fetch_optional<SE>(
        &mut self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> LunaOrmResult<Option<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_opt: Option<SE> = query.fetch_optional(self.get_pool()?).await?;
        Ok(result_opt)
    }

    async fn fetch_all<SE>(&mut self, stmt: &str, args: AnyArguments<'_>) -> LunaOrmResult<Vec<SE>>
    where
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_vec: Vec<SE> = query.fetch_all(self.get_pool()?).await?;
        Ok(result_vec)
    }

    async fn execute(
        &mut self,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> LunaOrmResult<AnyQueryResult> {
        Ok(sqlx::query_with(stmt, args)
            .execute(self.get_pool()?)
            .await?)
    }

    async fn execute_plain(&mut self, stmt: &str) -> LunaOrmResult<AnyQueryResult> {
        Ok(sqlx::query(stmt).execute(self.get_pool()?).await?)
    }
}
