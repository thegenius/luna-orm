use serde::{Deserialize, Serialize};
use sqlx::error::BoxDynError;
use sqlx::sqlite::{SqliteArguments, SqliteRow};
use sqlx::{Arguments, Row};
use sqlx::{Database, Sqlite};
use std::borrow::Cow;
use std::error::Error;
use taitan_orm::database::sqlite::SqliteDatabase;
use taitan_orm::SqlExecutor;
use taitan_orm_trait::{
    validate_order_by, Entity, Location, LocationExpr, LocationTrait, Mutation, OrderBy,
    SelectedEntity, Selection, Unique, UpdateCommand,
};
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub request_id: Uuid,
    pub name: String,
    pub age: Option<i32>,
    pub birthday: Option<PrimitiveDateTime>,
}

pub async fn prepare_user_table(db: &mut SqliteDatabase) -> taitan_orm::Result<()> {
    let _result = db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
    let _ = db
        .execute_plain(
            "CREATE TABLE IF NOT EXISTS `user`\
    (`id` BIGINT PRIMARY KEY, \
    `request_id` blob,  \
    `name` VARCHAR(64), \
    `age` INT, \
    `birthday` DATETIME)",
        )
        .await?;
    Ok(())
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
        let mut fields = Vec::new();
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
    pub id: i64,
}

impl Unique for UserPrimary {
    fn get_table_name(&self) -> &'static str {
        "user"
    }

    fn get_unique_field_names(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn gen_unique_arguments_sqlite(&self) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();
        args.add(&self.id)?;
        Ok(args)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserSelected {
    pub id: Option<u64>,
    pub request_id: Option<Uuid>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub birthday: Option<PrimitiveDateTime>,
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
    pub id: bool,
    pub request_id: bool,
    pub name: bool,
    pub age: bool,
    pub birthday: bool,
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

    fn full_fields() -> Self
    where
        Self: Sized,
    {
        Self {
            id: true,
            request_id: true,
            name: true,
            age: true,
            birthday: true,
        }
    }
}

#[derive(Debug)]
pub struct UserMutation {
    pub request_id: Option<Uuid>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub birthday: Option<PrimitiveDateTime>,
    // money: Option<BigDecimal>,
    // ipv4addr: Option<Ipv4Addr>,
    // ipv6addr: Option<Ipv6Addr>,
}

impl Mutation for UserMutation {
    type Primary = UserPrimary;
    type Location = UserLocation;
    fn get_mutation_fields_name(&self) -> Vec<String> {
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

    fn gen_update_arguments_sqlite<'a>(
        &'a self,
        primary: &'a Self::Primary,
    ) -> Result<SqliteArguments<'a>, BoxDynError> {
        let mut args = SqliteArguments::default();
        if let Some(request_id) = &self.request_id {
            args.add(request_id)?;
        }
        if let Some(name) = &self.name {
            args.add(name)?;
        }
        if let Some(age) = &self.age {
            args.add(age)?;
        }
        if let Some(birthday) = &self.birthday {
            args.add(birthday)?;
        }
        args.add(primary.id)?;
        Ok(args)
    }
}

#[derive(Debug)]
pub struct UserPrimaryMutationPair<'a>(pub &'a UserMutation, pub &'a UserPrimary);

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
    pub request_id: Option<LocationExpr<Uuid>>,
    pub name: Option<LocationExpr<String>>,
    pub age: Option<LocationExpr<i32>>,
    pub birthday: Option<LocationExpr<PrimitiveDateTime>>,
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

    fn get_location_fields_name(&self) -> Vec<String> {
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
pub struct UserLocationMutationPair(pub UserMutation, pub UserLocation);

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

#[derive(Debug, Default)]
pub struct UserOrderBy<'a> {
    fields: Vec<Cow<'a, str>>,
}

impl<'a> OrderBy for UserOrderBy<'a> {
    fn unique_fields(&self) -> &[&[&str]] {
        &[&["id"]]
    }

    fn get_fields(&self) -> &[Cow<'a, str>] {
        &self.fields
    }
}

impl<'a> UserOrderBy<'a> {
    pub fn build<I, S>(fields: I) -> Result<Self, Box<dyn Error + 'static>>
    where
        I: IntoIterator<Item = S> + Clone,
        S: AsRef<str> + Into<Cow<'a, str>>, // 确保每个元素可以转换为 Cow<'a, str>
    {
        let order_by = Self::default();
        validate_order_by(fields.clone(), order_by.unique_fields())?;

        Ok(Self {
            fields: fields.into_iter().map(Into::into).collect(),
        })
    }
}
