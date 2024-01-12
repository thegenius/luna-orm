use crate::database::lib::Database;
use crate::database::lib::DatabaseType;
use crate::database::DB;
use crate::{error::LunaOrmError, LunaOrmResult};

use sqlx::any::AnyConnectOptions;
use sqlx::AnyPool;

use crate::command_executor::CommandExecutor;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::PostgresGenerator;
use crate::sql_generator::SqlGenerator;
use luna_orm_trait::Entity;
use luna_orm_trait::LastRowId;
use std::str::FromStr;
use tracing::debug;

#[derive(Debug)]
pub struct PostgresDatabase {
    database_type: DatabaseType,
    pool: AnyPool,
    sql_generator: PostgresGenerator,
}

impl SqlExecutor for PostgresDatabase {
    fn get_pool(&self) -> LunaOrmResult<&AnyPool> {
        Ok(&self.pool)
    }
}

impl CommandExecutor for PostgresDatabase {
    type G = PostgresGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.sql_generator
    }

    async fn create<'a>(&mut self, entity: &'a mut dyn Entity) -> LunaOrmResult<bool> {
        debug!(target: "luna_orm", command = "create",  entity = ?entity);
        let sql = self.get_generator().get_create_sql(entity);
        debug!(target: "luna_orm", command = "create", sql = sql);
        let args = entity.any_arguments_of_insert();
        if entity.get_auto_increment_field().is_some() {
            let last_row_id: LastRowId = self.fetch_one(&sql, args).await?;
            entity.set_auto_increment_field(Some(last_row_id.id));
        } else {
            self.execute(&sql, args).await?;
        }
        debug!(target: "luna_orm", command = "create", result = ?entity);
        return Ok(true);
    }
}

impl Database for PostgresDatabase {
    fn get_type(&self) -> &DatabaseType {
        &self.database_type
    }
}

impl From<PostgresDatabase> for DB<PostgresDatabase> {
    fn from(value: PostgresDatabase) -> Self {
        Self(value)
    }
}

impl PostgresDatabase {
    pub async fn build(url: &str, user: &str, password: &str) -> LunaOrmResult<Self> {
        let url = format!("postgres://{}:{}@{}", user, password, url);

        let any_options = AnyConnectOptions::from_str(&url).unwrap();
        let pool = AnyPool::connect_with(any_options)
            .await
            .map_err(|_e| LunaOrmError::DatabaseInitFail("init pool fail".to_string()))?;

        let generator = PostgresGenerator::new();
        let database = PostgresDatabase {
            database_type: DatabaseType::MySql,
            pool,
            sql_generator: generator,
        };
        Ok(database)
    }
}
