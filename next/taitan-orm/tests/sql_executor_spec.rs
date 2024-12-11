//! | Rust type                             | MySQL/MariaDB type(s)                                |
//! |---------------------------------------|------------------------------------------------------|
//! | `bool`                                | TINYINT(1), BOOLEAN, BOOL (see below)                |
//! | `i8`                                  | TINYINT                                              |
//! | `i16`                                 | SMALLINT                                             |
//! | `i32`                                 | INT                                                  |
//! | `i64`                                 | BIGINT                                               |
//! | `u8`                                  | TINYINT UNSIGNED                                     |
//! | `u16`                                 | SMALLINT UNSIGNED                                    |
//! | `u32`                                 | INT UNSIGNED                                         |
//! | `u64`                                 | BIGINT UNSIGNED                                      |
//! | `f32`                                 | FLOAT                                                |
//! | `f64`                                 | DOUBLE                                               |
//! | `&str`, [`String`]                    | VARCHAR, CHAR, TEXT                                  |
//! | `&[u8]`, `Vec<u8>`                    | VARBINARY, BINARY, BLOB                              |
//! | `IpAddr`                              | VARCHAR, TEXT                                        |
//! | `Ipv4Addr`                            | INET4 (MariaDB-only), VARCHAR, TEXT                  |
//! | `Ipv6Addr`                            | INET6 (MariaDB-only), VARCHAR, TEXT                  |
//! | [`MySqlTime`]                         | TIME (encode and decode full range)                  |
//! | [`Duration`][std::time::Duration]     | TIME (for decoding positive values only)             |

//! | Rust type                             | MySQL/MariaDB type(s)                                |
//! |---------------------------------------|------------------------------------------------------|
//! | `time::PrimitiveDateTime`             | DATETIME                                             |
//! | `time::OffsetDateTime`                | TIMESTAMP                                            |
//! | `time::Date`                          | DATE                                                 |
//! | `time::Time`                          | TIME (time-of-day only)                              |
//! | `time::Duration`                      | TIME (decodes full range; see note for encoding)     |

//! | Rust type                             | MySQL/MariaDB type(s)                                |
//! |---------------------------------------|------------------------------------------------------|
//! | `bigdecimal::BigDecimal`              | DECIMAL                                              |

//! | Rust type                             | MySQL/MariaDB type(s)                                |
//! |---------------------------------------|------------------------------------------------------|
//! | `uuid::Uuid`                          | BINARY(16) (see note)                                |
//! | `uuid::fmt::Hyphenated`               | CHAR(36), VARCHAR, TEXT, UUID (MariaDB-only)         |
//! | `uuid::fmt::Simple`                   | CHAR(32), VARCHAR, TEXT                              |

use std::marker::PhantomData;
use taitan_orm_trait::{CmpOperator, LocationExpr, LocationTrait, Selection};
use taitan_orm_trait::{Entity, Location, Mutation, Primary, SelectedEntity, UpdateCommand};

use serde::{Deserialize, Serialize};
use sqlx::error::BoxDynError;

use sqlx::sqlite::SqliteArguments;
use sqlx::types::time::PrimitiveDateTime;

use sqlx::types::Uuid;
use sqlx::{sqlx_macros, Database, Sqlite};

use sqlx::Arguments;
use sqlx::Row;
use taitan_orm::database::sqlite::{SqliteCommander, SqliteLocalConfig};
use taitan_orm::{SqlExecutor, DB};
use time::macros::datetime;

#[derive(Debug)]
pub struct User {
    id: i64,
    request_id: Uuid,
    name: String,
    age: Option<i32>,
    birthday: Option<PrimitiveDateTime>,
}

impl Entity for User {
    fn get_table_name(&self) -> &str {
        "user"
    }

    fn get_insert_fields(&self) -> Vec<String> {
        let mut fields = Vec::new();
        fields.push("id".to_string());
        fields.push("request_id".to_string());
        fields.push("name".to_string());
        if let Some(_) = &self.age {
            fields.push("age".to_string());
        }
        if let Some(_) = &self.birthday {
            fields.push("birthday".to_string());
        }
        fields
    }

    fn get_upsert_set_fields(&self) -> Vec<String> {
        todo!()
    }

    fn get_auto_increment_field(&self) -> Option<&str> {
        todo!()
    }

    fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool {
        todo!()
    }

    fn gen_insert_arguments_sqlite(&self) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();
        args.add(&self.id)?;
        args.add(&self.request_id)?;
        args.add(&self.name)?;
        if let Some(age) = &self.age {
            args.add(age)?;
        }
        if let Some(birthday) = &self.birthday {
            args.add(birthday)?;
        }
        Ok(args)
    }

    fn gen_upsert_arguments_sqlite(&self) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();
        args.add(&self.id)?;

        args.add(&self.request_id)?;
        args.add(&self.name)?;
        if let Some(age) = &self.age {
            args.add(age)?;
        }
        if let Some(birthday) = &self.birthday {
            args.add(birthday)?;
        }

        args.add(&self.request_id)?;
        args.add(&self.name)?;
        if let Some(age) = &self.age {
            args.add(age)?;
        }
        if let Some(birthday) = &self.birthday {
            args.add(birthday)?;
        }
        Ok(args)
    }
}

#[derive(Debug)]
pub struct UserPrimary {
    id: i64,
}

impl Primary for UserPrimary {
    fn get_table_name(&self) -> &'static str {
        "user"
    }

    fn get_primary_field_names(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn gen_primary_arguments_sqlite(
        &self,
    ) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();
        args.add(&self.id)?;
        Ok(args)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserSelected {
    id: Option<u64>,
    request_id: Option<Uuid>,
    name: Option<String>,
    age: Option<i32>,
    birthday: Option<PrimitiveDateTime>,
    // money: Option<BigDecimal>,
    // ipv4addr: Option<Ipv4Addr>,
    // ipv6addr: Option<Ipv6Addr>,
}

impl SelectedEntity<Sqlite> for UserSelected {
    type Selection = UserSelection;

    fn from_row(
        selection: &Self::Selection,
        row: <Sqlite as Database>::Row,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        if selection.id {
            selected.id = row.try_get("id").ok();
        }
        if selection.request_id {
            selected.request_id = row.try_get("request_id").ok();
        }
        if selection.name {
            selected.name = row.try_get("name").ok();
        }
        if selection.age {
            selected.age = row.try_get("age").ok();
        }
        if selection.birthday {
            selected.birthday = row.try_get("birthday").ok();
        }
        Ok(selected)
    }
}

#[derive(Debug, Default)]
pub struct UserSelection {
    id: bool,
    request_id: bool,
    name: bool,
    age: bool,
    birthday: bool,
    // money: bool,
    // ipv4addr: bool,
    // ipv6addr: bool,
}

impl Selection for UserSelection {
    fn get_table_name(&self) -> &'static str {
        "user"
    }

    fn get_selected_fields(&self) -> Vec<String> {
        let mut fields = Vec::new();
        if self.id {
            fields.push("id".to_string());
        }
        if self.request_id {
            fields.push("request_id".to_string());
        }
        if self.name {
            fields.push("name".to_string());
        }
        if self.age {
            fields.push("age".to_string());
        }
        if self.birthday {
            fields.push("birthday".to_string());
        }
        fields
    }
}

#[derive(Debug)]
pub struct UserMutation {
    request_id: Option<Uuid>,
    name: Option<String>,
    age: Option<i32>,
    birthday: Option<PrimitiveDateTime>,
    // money: Option<BigDecimal>,
    // ipv4addr: Option<Ipv4Addr>,
    // ipv6addr: Option<Ipv6Addr>,
}

impl Mutation for UserMutation {
    type Primary = UserPrimary;
    type Location = UserLocation;
    fn get_fields_name(&self) -> Vec<String> {
        let mut fields = Vec::new();
        if let Some(_) = &self.request_id {
            fields.push("request_id".to_string());
        }
        if let Some(_) = &self.name {
            fields.push("name".to_string());
        }
        if let Some(_) = &self.age {
            fields.push("age".to_string());
        }
        if let Some(_) = &self.birthday {
            fields.push("birthday".to_string());
        }
        // if let Some(_) = &self.money {
        //     fields.push("money".to_string());
        // }
        // if let Some(_) = &self.ipv4addr {
        //     fields.push("ipv4addr".to_string());
        // }
        // if let Some(_) = &self.ipv6addr {
        //     fields.push("ipv6addr".to_string());
        // }
        fields
    }
}

#[derive(Debug)]
pub struct UserPrimaryMutationPair<'a>(&'a UserMutation, &'a UserPrimary);

impl<'a> UpdateCommand for UserPrimaryMutationPair<'a> {
    fn gen_update_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();

        if let Some(request_id) = &self.0.request_id {
            args.add(request_id)?;
        }
        if let Some(name) = &self.0.name {
            args.add(name)?;
        }
        if let Some(age) = &self.0.age {
            args.add(age)?;
        }
        if let Some(birthday) = &self.0.birthday {
            args.add(birthday)?;
        }

        args.add(&self.1.id)?;

        Ok(args)
    }
}

#[derive(Debug)]
pub struct UserLocation {
    request_id: Option<LocationExpr<Uuid>>,
    name: Option<LocationExpr<String>>,
    age: Option<LocationExpr<i32>>,
    birthday: Option<LocationExpr<PrimitiveDateTime>>,
    // money: LocationExpr<BigDecimal>,
    // ipv4addr: LocationExpr<Ipv4Addr>,
    // ipv6addr: LocationExpr<Ipv6Addr>,
}

impl UserLocation {
    #[inline(always)]
    pub fn concat_where_clause<T>(
        &self,
        sql: &mut String,
        wrap_char: char,
        place_holder: char,
        field_name: &str,
        loc: &dyn LocationTrait,
    ) {
        sql.push(wrap_char);
        sql.push_str(field_name);
        sql.push(wrap_char);
        sql.push_str(loc.get_cmp_sql());
        sql.push(place_holder);
    }
}

impl Location for UserLocation {
    fn get_table_name(&self) -> &'static str {
        "user"
    }

    fn get_fields_name(&self) -> Vec<String> {
        let mut fields = Vec::new();
        if let Some(_) = &self.request_id {
            fields.push("request_id".to_string());
        }
        if let Some(_) = &self.name {
            fields.push("name".to_string());
        }
        if let Some(_) = &self.age {
            fields.push("age".to_string());
        }
        if let Some(_) = &self.birthday {
            fields.push("birthday".to_string());
        }
        // if let Some(_) = &self.money {
        //     fields.push("money".to_string());
        // }
        // if let Some(_) = &self.ipv4addr {
        //     fields.push("ipv4addr".to_string());
        // }
        // if let Some(_) = &self.ipv6addr {
        //     fields.push("ipv6addr".to_string());
        // }
        fields
    }

    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
        let mut sql = String::default();
        if let Some(request_id) = &self.request_id {
            sql.push(wrap_char);
            sql.push_str("request_id");
            sql.push(wrap_char);
            sql.push_str(request_id.get_cmp_sql());
            sql.push(place_holder);
        }
        if let Some(name) = &self.name {
            sql.push(wrap_char);
            sql.push_str("name");
            sql.push(wrap_char);
            sql.push_str(name.get_cmp_sql());
            sql.push(place_holder);
        }
        if let Some(age) = &self.age {
            sql.push(wrap_char);
            sql.push_str("age");
            sql.push(wrap_char);
            sql.push_str(age.get_cmp_sql());
            sql.push(place_holder);
        }
        if let Some(birthday) = &self.birthday {
            sql.push(wrap_char);
            sql.push_str("birthday");
            sql.push(wrap_char);
            sql.push_str(birthday.get_cmp_sql());
            sql.push(place_holder);
        }
        sql
    }

    fn check_valid_order_by(&self, fields: &[&str]) -> bool {
        todo!()
    }

    fn gen_location_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();

        if let Some(request_id) = &self.request_id {
            args.add(&request_id.val)?;
        }
        if let Some(name) = &self.name {
            args.add(&name.val)?;
        }
        if let Some(age) = &self.age {
            args.add(&age.val)?;
        }
        if let Some(birthday) = &self.birthday {
            args.add(&birthday.val)?;
        }

        Ok(args)
    }
    //
    // fn gen_change_arguments_sqlite<'a>(
    //     &'a self,
    //     mutation: &'a Self::Mutation,
    // ) -> Result<SqliteArguments<'a>, BoxDynError> {
    //     let mut args = SqliteArguments::default();
    //
    //     if let Some(request_id) = &mutation.request_id {
    //         args.add(request_id)?;
    //     }
    //     if let Some(name) = &mutation.name {
    //         args.add(name)?;
    //     }
    //     if let Some(age) = &mutation.age {
    //         args.add(age)?;
    //     }
    //     if let Some(birthday) = &mutation.birthday {
    //         args.add(birthday)?;
    //     }
    //
    //     if let Some(request_id) = &self.request_id {
    //         args.add(request_id.val)?;
    //     }
    //     if let Some(name) = &self.name {
    //         args.add(name.clone().val)?;
    //     }
    //     if let Some(age) = &self.age {
    //         args.add(age.val)?;
    //     }
    //     if let Some(birthday) = &self.birthday {
    //         args.add(birthday.val)?;
    //     }
    //
    //     Ok(args)
    // }
}

#[derive(Debug)]
pub struct UserLocationMutationPair(UserMutation, UserLocation);

impl UpdateCommand for UserLocationMutationPair {
    fn gen_update_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();

        if let Some(request_id) = &self.0.request_id {
            args.add(request_id)?;
        }
        if let Some(name) = &self.0.name {
            args.add(name)?;
        }
        if let Some(age) = &self.0.age {
            args.add(age)?;
        }
        if let Some(birthday) = &self.0.birthday {
            args.add(birthday)?;
        }

        if let Some(request_id) = &self.1.request_id {
            args.add(request_id.val)?;
        }
        if let Some(name) = &self.1.name {
            args.add(name.clone().val)?;
        }
        if let Some(age) = &self.1.age {
            args.add(age.val)?;
        }
        if let Some(birthday) = &self.1.birthday {
            args.add(birthday.val)?;
        }

        Ok(args)
    }
}

async fn test_insert_user(db: &mut DB<SqliteCommander>, user: &User) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;

    let args = user.gen_insert_arguments_sqlite().unwrap();

    let result = db.generic_execute(&mut *conn, "INSERT INTO `user`(`id`, `request_id`, `name`, `age`, `birthday`) VALUES(?, ?, ?, ?, ?)",
                                args).await?;
    assert_eq!(result, 1);

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;
    let primary = UserPrimary { id: user.id };
    let primary_args = primary.gen_primary_arguments_sqlite().unwrap();
    let entity_opt: Option<UserSelected> = db
        .generic_fetch_optional(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `id` = ?",
            &selection,
            primary_args,
        )
        .await?;

    assert!(entity_opt.is_some());
    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
    assert_eq!(selected_entity.name.unwrap(), user.name);
    assert_eq!(selected_entity.age, user.age);
    assert_eq!(selected_entity.birthday, user.birthday);
    Ok(())
}

/**
因为UPDATE语句固定了，所以目前要求mutation必须包含所有字段
*/
async fn test_update_user(
    db: &mut DB<SqliteCommander>,
    user_mutation: &UserMutation,
    user_primary: &UserPrimary,
) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;
    let update_command = UserPrimaryMutationPair(user_mutation, user_primary);
    let args = update_command.gen_update_arguments_sqlite().unwrap();
    // let args: SqliteArguments = user_primary
    //     .gen_update_arguments_sqlite(user_mutation)
    //     .unwrap();
    let result = db.generic_execute(&mut *conn, "UPDATE `user` SET `request_id` = ?, `name` = ?, `age` = ?, `birthday` = ? WHERE `id` = ?",
                                args).await?;
    assert_eq!(result, 1);

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let primary_args = user_primary.gen_primary_arguments_sqlite().unwrap();
    let entity_opt: Option<UserSelected> = db
        .generic_fetch_optional(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `id` = ?",
            &selection,
            primary_args,
        )
        .await?;

    assert!(entity_opt.is_some());
    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id, user_mutation.request_id);
    assert_eq!(selected_entity.name, user_mutation.name);
    assert_eq!(selected_entity.age, user_mutation.age);
    assert_eq!(selected_entity.birthday, user_mutation.birthday);
    Ok(())
}

async fn test_upsert_user(db: &mut DB<SqliteCommander>, user: &User) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;
    let args: SqliteArguments = user.gen_upsert_arguments_sqlite().unwrap();
    let result = db.generic_execute(&mut *conn, "INSERT INTO `user`(`id`, `request_id`, `name`, `age`, `birthday`) VALUES (?, ?, ?, ?, ?)
ON CONFLICT (`id`) DO UPDATE SET
`request_id` = ?, `name` = ?, `age` = ?, `birthday` = ?", args).await?;
    assert_eq!(result, 1);

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let user_primary: UserPrimary = UserPrimary { id: user.id };
    let primary_args = user_primary.gen_primary_arguments_sqlite().unwrap();
    let entity_opt: Option<UserSelected> = db
        .generic_fetch_optional(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `id` = ?",
            &selection,
            primary_args,
        )
        .await?;

    assert!(entity_opt.is_some());
    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
    assert_eq!(selected_entity.name.unwrap(), user.name);
    assert_eq!(selected_entity.age, user.age);
    assert_eq!(selected_entity.birthday, user.birthday);
    Ok(())
}

async fn test_delete_user(
    db: &mut DB<SqliteCommander>,
    user_primary: &UserPrimary,
) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;
    let args: SqliteArguments = user_primary.gen_primary_arguments_sqlite().unwrap();
    let result = db
        .generic_execute(&mut *conn, "DELETE FROM `user` WHERE `id` = ?", args)
        .await?;
    assert_eq!(result, 1);

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let user_primary: UserPrimary = UserPrimary {
        id: user_primary.id,
    };
    let primary_args = user_primary.gen_primary_arguments_sqlite().unwrap();
    let entity_opt: Option<UserSelected> = db
        .generic_fetch_optional(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `id` = ?",
            &selection,
            primary_args,
        )
        .await?;

    assert!(entity_opt.is_none());
    Ok(())
}

async fn test_select_all(
    db: &mut DB<SqliteCommander>,
    expect_cnt: usize,
) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let phantom = PhantomData::<SqliteArguments>::default();
    let entity_vec: Vec<UserSelected> = db
        .generic_fetch_all_plain(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user`",
            &selection,
            phantom,
        )
        .await?;

    assert_eq!(entity_vec.len(), expect_cnt);
    Ok(())
}

async fn test_select_location(
    db: &mut DB<SqliteCommander>,
    user_location: &UserLocation,
    expect_cnt: usize,
) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;
    let loc_args: SqliteArguments = user_location.gen_location_arguments_sqlite().unwrap();

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let entities: Vec<UserSelected> = db
        .generic_fetch_all(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `birthday` = ?",
            &selection,
            loc_args,
        )
        .await?;

    assert_eq!(entities.len(), expect_cnt);
    Ok(())
}

/**
测试用例设计如下：
insert  id = 1
update  id = 1
upsert  id = 2

select all
select id = 2
delete id = 2
select all where id = 1
select all where id = 2
*/
#[sqlx_macros::test]
pub async fn sql_executor_spec() -> taitan_orm::Result<()> {
    let config = SqliteLocalConfig {
        work_dir: "./workspace".into(),
        db_file: "test.db".into(),
    };

    let mut db: DB<SqliteCommander> = SqliteCommander::build(config).await.unwrap().into();
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await.unwrap();
    let result = db
        .generic_execute_plain(
            &mut *conn,
            "DROP TABLE IF EXISTS `user`",
            PhantomData::<SqliteArguments>::default(),
        )
        .await?;
    let result = db.generic_execute_plain(&mut *conn,
        "CREATE TABLE IF NOT EXISTS `user`(`id` BIGINT PRIMARY KEY, `request_id` blob,  `name` VARCHAR(64), `age` INT, `birthday` DATETIME)",
                                      PhantomData::<SqliteArguments>::default()).await?;

    let entity1 = User {
        id: 1,
        request_id: Uuid::new_v4(),
        name: "Allen".to_string(),
        age: Some(23),
        birthday: Some(datetime!(2019-01-01 0:00)),
    };

    test_insert_user(&mut db.clone(), &entity1).await?;

    let mutaion1 = UserMutation {
        request_id: Some(Uuid::new_v4()),
        name: Some("Allen Woods".to_string()),
        age: Some(25),
        birthday: Some(datetime!(2019-01-02 0:00)),
    };
    let primary1 = UserPrimary { id: 1 };
    test_update_user(&mut db.clone(), &mutaion1, &primary1).await?;

    let entity1 = User {
        id: 1,
        request_id: Uuid::new_v4(),
        name: "Bob".to_string(),
        age: Some(24),
        birthday: Some(datetime!(2020-01-03 12:59)),
    };
    test_upsert_user(&mut db.clone(), &entity1).await?;

    let entity1 = User {
        id: 2,
        request_id: Uuid::new_v4(),
        name: "Bob Woods".to_string(),
        age: Some(24),
        birthday: Some(datetime!(2020-01-01 0:00)),
    };
    test_insert_user(&mut db.clone(), &entity1).await?;
    test_select_all(&mut db.clone(), 2).await?;

    let user_primary2 = UserPrimary { id: 2 };
    test_delete_user(&mut db.clone(), &user_primary2).await?;
    test_select_all(&mut db.clone(), 1).await?;

    let user_location: UserLocation = UserLocation {
        request_id: None,
        name: None,
        age: None,
        birthday: Some(LocationExpr::new(
            CmpOperator::Eq,
            datetime!(2020-01-03 12:59),
        )),
    };
    test_select_location(&mut db.clone(), &user_location, 1).await?;

    Ok(())
}
