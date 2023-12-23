use crate::database::lib::Database;
use crate::database::lib::DatabaseType;
use crate::database::DB;
use crate::{error::LunaOrmError, LunaOrmResult};

use sqlx::any::AnyConnectOptions;
use sqlx::AnyPool;

use std::str::FromStr;

use crate::command_executor::CommandExecutor;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::PostgresGenerator;

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
