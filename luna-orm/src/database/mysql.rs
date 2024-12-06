mod MySqlInputGenerator;

use crate::database::lib::Database;
use crate::database::lib::DatabaseType;
use crate::database::DB;
use crate::{error::LunaOrmError, LunaOrmResult};

use sqlx::any::AnyConnectOptions;
use sqlx::AnyPool;

use crate::command_executor::CommandExecutor;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::MySqlGenerator;
use crate::sql_generator::SqlGenerator;
use luna_orm_trait::Entity;
use std::str::FromStr;
use tracing::debug;

#[derive(Debug)]
pub struct MysqlDatabase {
    database_type: DatabaseType,
    pool: AnyPool,
    sql_generator: MySqlGenerator,
}

impl SqlExecutor for MysqlDatabase {
    fn get_pool(&self) -> LunaOrmResult<&AnyPool> {
        Ok(&self.pool)
    }
}

impl CommandExecutor for MysqlDatabase {
    type G = MySqlGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.sql_generator
    }

    async fn create<'a>(&mut self, entity: &'a mut dyn Entity) -> LunaOrmResult<bool> {
        debug!(target: "luna_orm", command = "create",  entity = ?entity);
        let sql = self.get_generator().get_insert_sql(entity);
        debug!(target: "luna_orm", command = "create", sql = sql);
        let args = entity.any_arguments_of_insert();
        let result = self.execute(&sql, args).await?;
        entity.set_auto_increment_field(result.last_insert_id());
        debug!(target: "luna_orm", command = "create", result = ?entity);
        return Ok(result.rows_affected() > 0);
    }
}

impl Database for MysqlDatabase {
    fn get_type(&self) -> &DatabaseType {
        &self.database_type
    }
}

impl From<MysqlDatabase> for DB<MysqlDatabase> {
    fn from(value: MysqlDatabase) -> Self {
        Self(value)
    }
}

impl MysqlDatabase {
    pub async fn build(url: &str, user: &str, password: &str) -> LunaOrmResult<Self> {
        let url = format!("mysql://{}:{}@{}", user, password, url);

        let any_options = AnyConnectOptions::from_str(&url).unwrap();
        let pool = AnyPool::connect_with(any_options)
            .await
            .map_err(|_e| LunaOrmError::DatabaseInitFail("init pool fail".to_string()))?;

        let generator = MySqlGenerator::new();
        let database = MysqlDatabase {
            database_type: DatabaseType::MySql,
            pool,
            sql_generator: generator,
        };
        return Ok(database);
    }
}
