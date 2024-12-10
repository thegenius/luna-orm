use luna_orm::LunaOrmResult;
use luna_orm::prelude::{Entity, InputGenerator, Location, Mutation, OrderBy, Primary};
use luna_orm::prelude::{CommandExecutor, SqlExecutor, SqliteDatabase, SqliteLocalConfig, DB};
use sqlx::{Arguments, Database, Sqlite};
use sqlx::sqlite::SqliteArguments;
use sqlx::error::BoxDynError;
pub struct UserEntity {
    id: i32,
    name: String,
    age: Option<i32>,
}



impl InputGenerator<Sqlite> for UserEntity {
    fn gen_insert_arguments(&self, entity: &dyn Entity) -> Result<Sqlite::Arguments<'_>, BoxDynError> {
        let mut arguments: <Sqlite as Database>::Arguments<'_> = SqliteArguments::default();
        arguments.add(&self.name)?;
        if let Some(age) = self.age {
            arguments.add(age)?;
        }
        Ok(arguments)
    }

    fn gen_upsert_arguments(&self, entity: &dyn Entity) -> <Sqlite as Database>::Arguments<'_> {
        todo!()
    }

    fn gen_update_arguments(&self, mutation: &dyn Mutation, primary: &dyn Primary) -> <Sqlite as Database>::Arguments<'_> {
        todo!()
    }

    fn gen_change_arguments(&self, mutation: &dyn Mutation, location: &dyn Location) -> <Sqlite as Database>::Arguments<'_> {
        todo!()
    }

    fn gen_primary_arguments(&self, primary: &dyn Primary) -> <Sqlite as Database>::Arguments<'_> {
        todo!()
    }

    fn gen_location_arguments(&self, location: &dyn Location, order_by_option: Option<&dyn OrderBy>) -> <Sqlite as Database>::Arguments<'_> {
        todo!()
    }
}


#[tokio::main]
async fn main() -> LunaOrmResult<()> {
    let config = SqliteLocalConfig {
        work_dir: "./workspace".to_string(),
        db_file: "test.db".to_string(),
    };

    let mut db: DB<SqliteDatabase> = SqliteDatabase::build(config).await.unwrap().into();

    let result = db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
    let result = db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
    ).await?;

    // 1. insert entity
    let entity = UserEntity {
        id: 1,
        name: "Allen".to_string(),
        age: Some(23),
    };

    let insert_stmt: &str = "";
    let args = entity.gen_insert_arguments(&entity).unwrap();
    let result = db.execute_with_db(insert_stmt, args).await?;


    Ok(())
}