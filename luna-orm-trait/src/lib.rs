#![allow(dead_code)]
use async_trait::async_trait;
use sqlx::any::AnyArguments;
use sqlx::any::AnyRow;
use sqlx::Any;
use sqlx::AnyPool;
use sqlx::Arguments;
use sqlx::Database;
use sqlx::Encode;
use sqlx::Row;
use sqlx::Type;

use serde::{Deserialize, Serialize};

mod field;
pub mod lib2;
mod location;

pub type SqlxError = sqlx::Error;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LocationExpr<T> {
    pub val: T,
    pub cmp: CmpOperator,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum CmpOperator {
    #[serde(alias = "=")]
    Eq,
    #[serde(alias = "<")]
    LessThan,
    #[serde(alias = "<=")]
    LessOrEq,
    #[serde(alias = ">")]
    GreaterThan,
    #[serde(alias = ">=")]
    GreaterOrEq,
    #[serde(alias = "like")]
    Like,
}

impl CmpOperator {
    pub fn get_sql(&self) -> &'static str {
        match self {
            CmpOperator::Eq => "=",
            CmpOperator::LessThan => "<",
            CmpOperator::LessOrEq => "<=",
            CmpOperator::GreaterThan => ">",
            CmpOperator::GreaterOrEq => ">=",
            CmpOperator::Like => "LIKE",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pagination {
    pub page_size: usize,
    pub page_num: usize,
}

#[derive(Clone, Debug)]
pub struct PageInfo {
    pub page_size: usize,
    pub page_num: usize,
    pub page_total: usize,
    pub total: usize,
}

#[derive(Clone, Debug)]
pub struct PagedList<T>
where
    T: SelectedEntity,
{
    pub data: Vec<T>,
    pub page: PageInfo,
}

pub fn merge_any_arguments<'p>(
    mut args_a: AnyArguments<'p>,
    args_b: AnyArguments<'p>,
) -> AnyArguments<'p> {
    args_a.values.0.extend(args_b.values.0);
    args_a
}

pub fn add_arg<'q, T>(args: &mut AnyArguments<'q>, value: &T)
where
    T: 'q + Send + Encode<'q, Any> + Type<Any>,
{
    let _ = value.encode_by_ref(&mut args.values);
}

pub trait Primary: Sync {
    fn get_table_name(&self) -> &'static str;
    fn get_primary_field_names(&self) -> &'static [&'static str];

    fn table_name(&self, wrap_char: char) -> String {
        let name = self.name();
        return format!("{}{}{}", wrap_char, name.to_lowercase(), wrap_char);
    }

    fn name(&self) -> String;

    fn get_fields_name(&self) -> Vec<String>;

    fn get_where_clause(&self, wrap_char: char, place_holder: &str) -> String {
        let fields = self.get_fields_name();
        let sql_fields: Vec<String> = fields
            .iter()
            .map(|e| format!("{}{}{} = {}", wrap_char, e, wrap_char, place_holder))
            .collect();
        return sql_fields.join(" ,");
    }
    /*
    fn into_any_arguments<'p>(self) -> AnyArguments<'p>
    where
        Self: Sized;
    */

    fn any_arguments(&self) -> AnyArguments<'_>;
}

pub trait Mutation: Sync {
    fn into_any_arguments<'p>(self) -> AnyArguments<'p>
    where
        Self: Sized;

    fn any_arguments<'p>(&self) -> AnyArguments<'p>;

    fn get_fields_name(&self) -> Vec<String>;

    fn get_update_clause(&self, wrap_char: char, place_holder: char) -> String {
        let fields = self.get_fields_name();
        let sql_fields: Vec<String> = fields
            .iter()
            .map(|e| format!("{}{}{} = {}", wrap_char, e, wrap_char, place_holder))
            .collect();
        return sql_fields.join(" ,");
    }
}

pub trait Location: Sync {
    fn name(&self) -> String;

    fn get_table_name(&self) -> &'static str;

    fn into_any_arguments<'p>(self) -> AnyArguments<'p>
    where
        Self: Sized;

    fn any_arguments<'p>(&self) -> AnyArguments<'p>;

    fn get_fields_name(&self) -> Vec<String>;

    fn table_name(&self, wrap_char: char) -> String {
        let name = self.name();
        return format!("{}{}{}", wrap_char, name.to_lowercase(), wrap_char);
    }

    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String;
}

pub trait SelectedEntity2 {
    fn get_select_sql(&self, wrap_char: char, place_holder: &str) -> String;

    fn from_row<DB, T>(row: T) -> Result<Self, SqlxError>
    where
        DB: Database,
        T: Row<Database = DB>,
        Self: Sized;

    fn into_select_arguments<'p>(self) -> AnyArguments<'p>
    where
        Self: Sized;

    fn from_any_row(row: AnyRow) -> Result<Self, SqlxError>
    where
        Self: Sized;
}

pub trait Entity: Sync {
    fn table_name(&self, wrap_char: char) -> String {
        let name = self.name();
        return format!("{}{}{}", wrap_char, name.to_lowercase(), wrap_char);
    }

    fn name(&self) -> String;

    fn get_table_name(&self) -> &'static str;

    fn get_fields_name(&self) -> Vec<String> {
        let mut fields = self.get_primary_fields_name();
        let body_fields = self.get_body_fields_name();
        fields.extend(body_fields);
        return fields;
    }

    fn get_primary_fields_name(&self) -> Vec<String>;

    fn get_body_fields_name(&self) -> Vec<String>;

    fn any_arguments_of_insert<'p>(&self) -> AnyArguments<'p>;
    fn any_arguments_of_upsert<'p>(&self) -> AnyArguments<'p>;
    fn any_arguments_of_update<'p>(&self) -> AnyArguments<'p>;

    /*
    fn into_insert_any_arguments<'p>(self) -> AnyArguments<'p>
    where
        Self: Sized;

    fn into_update_any_arguments<'p>(self) -> AnyArguments<'p>
    where
        Self: Sized;

    fn into_upsert_any_arguments<'p>(self) -> AnyArguments<'p>
    where
        Self: Sized;
    */

    fn from_any_row(row: AnyRow) -> Result<Self, SqlxError>
    where
        Self: Sized;

    fn get_where_clause(&self, wrap_char: char, place_holder: &str) -> String {
        return self.get_assignment_clause(wrap_char, place_holder);
    }

    fn wrap_fields(
        &self,
        fields: &Vec<String>,
        wrap_char: char,
        assign_place_holder: Option<&str>,
    ) -> String {
        if let Some(place_holder) = assign_place_holder {
            fields
                .iter()
                .map(|e| format!("{}{}{} = {}", wrap_char, e, wrap_char, place_holder))
                .collect::<Vec<String>>()
                .join(",")
        } else {
            fields
                .iter()
                .map(|e| format!("{}{}{}", wrap_char, e, wrap_char))
                .collect::<Vec<String>>()
                .join(",")
        }
    }

    fn get_body_assignment_clause(&self, wrap_char: char, place_holder: &str) -> String {
        let fields = self.get_body_fields_name();
        return self.wrap_fields(&fields, wrap_char, Some(place_holder));
    }

    fn get_primary_assignment_clause(&self, wrap_char: char, place_holder: &str) -> String {
        let fields = self.get_primary_fields_name();
        return self.wrap_fields(&fields, wrap_char, Some(place_holder));
    }

    fn get_assignment_clause(&self, wrap_char: char, place_holder: &str) -> String {
        let fields = self.get_fields_name();
        return self.wrap_fields(&fields, wrap_char, Some(place_holder));
    }

    fn get_primary_fields_string(&self, wrap_char: char) -> String {
        let fields = self.get_primary_fields_name();
        return self.wrap_fields(&fields, wrap_char, None);
    }

    fn get_fields_string(&self, wrap_char: char) -> String {
        let fields = self.get_fields_name();
        return self.wrap_fields(&fields, wrap_char, None);
    }
}

pub trait Selection: Sync {
    fn get_selected_fields(&self) -> Vec<String>;
    fn get_sql_selection(&self, wrap_char: char) -> String {
        let selected_fields = self.get_selected_fields();
        let sql_fields: Vec<String> = selected_fields
            .iter()
            .map(|e| format!("{}{}{}", wrap_char, e, wrap_char))
            .collect();
        return sql_fields.join(" ,");
    }
}

pub trait SelectedEntity {
    fn from_any_row(row: AnyRow) -> Result<Self, SqlxError>
    where
        Self: Sized;
}

pub trait PrimarySync: Primary + Sync {}

pub trait AsPrimary {
    fn as_primary(&self) -> &dyn Primary;
}

impl<T: Primary> AsPrimary for T {
    fn as_primary(&self) -> &dyn Primary {
        self
    }
}

#[async_trait]
pub trait GenericDaoMapper {
    type P: Primary + Send;
    type E: Entity + Send + Clone;
    type S: Selection + Send;
    type SE: SelectedEntity + Send + Unpin;
    type L: Location + Send;
    type M: Mutation + Send;

    // fn get_executer<'a>(&'a self) -> dyn AnyExecutor;

    fn get_pool<'a>(&'a self) -> &'a AnyPool;

    async fn try_select(
        &self,
        primary: Self::P,
        selection: Self::S,
    ) -> Result<Option<Self::SE>, SqlxError> {
        // let mut transation = self.get_pool().begin().await?;

        let select_clause = selection.get_sql_selection('`');
        let where_clause = primary.get_where_clause('`', "?");
        let table_name = primary.table_name('`');
        let select_stmt = &format!(
            "SELECT {} FROM {} WHERE {}",
            select_clause, table_name, where_clause
        );
        // dbg!(&select_stmt);
        let args = primary.any_arguments();
        let sqlx_query =
            sqlx::query_with(select_stmt, args).try_map(|row: AnyRow| Self::SE::from_any_row(row));
        let entity_opt: Option<Self::SE> = sqlx_query.fetch_optional(self.get_pool()).await?;
        // let entity_opt: Option<Self::SE> = sqlx_query.fetch_optional(&mut *transation).await?;
        return Ok(entity_opt);
    }

    async fn try_create(&self, entity: Self::E) -> Result<Self::E, SqlxError> {
        let table_name = entity.table_name('`');
        let field_names = entity.get_fields_string('`');
        let question_marks = entity
            .get_fields_name()
            .iter()
            .map(|_| "?".to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let insert_stmt = &format!(
            "INSERT INTO {} ({}) VALUES({})",
            table_name, field_names, question_marks
        );
        let args = entity.clone().any_arguments_of_insert();
        let _ = sqlx::query_with(insert_stmt, args)
            .execute(self.get_pool())
            .await?;
        return Ok(entity);
    }

    async fn try_insert(&self, entity: Self::E) -> Result<bool, SqlxError> {
        let table_name = entity.table_name('`');
        let field_names = entity.get_fields_string('`');
        let question_marks = entity
            .get_fields_name()
            .iter()
            .map(|_| "?".to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let insert_stmt = &format!(
            "INSERT INTO {} ({}) VALUES({})",
            table_name, field_names, question_marks
        );
        let args = entity.clone().any_arguments_of_insert();
        let result = sqlx::query_with(insert_stmt, args)
            .execute(self.get_pool())
            .await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn try_upsert(&self, entity: Self::E) -> Result<bool, SqlxError> {
        let table_name = entity.table_name('`');
        let field_names = entity.get_fields_string('`');
        let question_marks = entity
            .get_fields_name()
            .iter()
            .map(|_| "?".to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let primary_fields_string = entity.get_primary_fields_string('`');
        let upsert_assign_clause = entity.get_body_assignment_clause('`', "?");
        let upsert_stmt = &format!(
            "INSERT INTO {}({}) VALUES({})
            ON CONFLICT({}) DO UPDATE SET {}",
            table_name, field_names, question_marks, primary_fields_string, upsert_assign_clause
        );
        let args = entity.clone().any_arguments_of_upsert();
        let result = sqlx::query_with(upsert_stmt, args)
            .execute(self.get_pool())
            .await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn try_update(&self, entity: Self::E) -> Result<bool, SqlxError> {
        let table_name = entity.table_name('`');
        let body_assign_clause = entity.get_body_assignment_clause('`', "?");
        let primary_assign_clause = entity.get_primary_assignment_clause('`', "?");
        let update_stmt = &format!(
            "UPDATE {} SET {} WHERE {}",
            table_name, body_assign_clause, primary_assign_clause
        );
        let args = entity.any_arguments_of_update();
        let result = sqlx::query_with(update_stmt, args)
            .execute(self.get_pool())
            .await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn remove(&self, _primary: Self::P) -> Result<Self::E, SqlxError> {
        todo!()
    }
    async fn try_delete(&self, primary: Self::P) -> Result<bool, SqlxError> {
        let table_name = primary.table_name('`');
        let where_clause = primary.get_where_clause('`', "?");
        let delete_stmt = &format!("DELETE FROM {} WHERE {}", table_name, where_clause);
        let args = primary.any_arguments();
        let result = sqlx::query_with(delete_stmt, args)
            .execute(self.get_pool())
            .await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn try_search(
        &self,
        location: Self::L,
        selection: Self::S,
    ) -> Result<Vec<Self::SE>, SqlxError> {
        let table_name = location.table_name('`');
        let selected_fields = selection.get_sql_selection('`');
        let where_clause = location.get_where_clause('`', '?');
        let search_stmt = &format!(
            "SELECT {} FROM {} WHERE {}",
            selected_fields, table_name, where_clause
        );
        let args = location.into_any_arguments();
        let sqlx_query =
            sqlx::query_with(search_stmt, args).try_map(|row: AnyRow| Self::SE::from_any_row(row));
        let entity_list = sqlx_query.fetch_all(self.get_pool()).await?;
        return Ok(entity_list);
    }

    async fn try_search_paged(
        &self,
        location: Self::L,
        selection: Self::S,
    ) -> Result<PagedList<Self::SE>, SqlxError> {
        let table_name = location.table_name('`');
        let selected_fields = selection.get_sql_selection('`');
        let where_clause = location.get_where_clause('`', '?');
        let search_stmt = &format!(
            "SELECT {} FROM {} WHERE {}",
            selected_fields, table_name, where_clause
        );
        let args = location.into_any_arguments();
        let sqlx_query =
            sqlx::query_with(search_stmt, args).try_map(|row: AnyRow| Self::SE::from_any_row(row));
        let entity_list = sqlx_query.fetch_all(self.get_pool()).await?;
        let page_info = PageInfo {
            page_size: 10,
            page_num: 10,
            page_total: 10,
            total: 100,
        };
        return Ok(PagedList {
            data: entity_list,
            page: page_info,
        });
    }

    async fn try_purify(&self, location: Self::L) -> Result<usize, SqlxError> {
        let table_name = location.table_name('`');
        let where_clause = location.get_where_clause('`', '?');
        let delete_stmt = &format!("DELETE FROM {} WHERE {}", table_name, where_clause);
        let args = location.into_any_arguments();
        let sqlx_query = sqlx::query_with(delete_stmt, args);
        let result = sqlx_query.execute(self.get_pool()).await?;
        return Ok(result.rows_affected() as usize);
    }

    async fn try_change(&self, location: Self::L, mutation: Self::M) -> Result<usize, SqlxError> {
        let table_name = location.table_name('`');
        let update_clause = mutation.get_update_clause('`', '?');
        let where_clause = location.get_where_clause('`', '?');
        let change_stmt = &format!(
            "UPDATE {} SET {} WHERE {}",
            table_name, update_clause, where_clause
        );
        let mutation_args = mutation.into_any_arguments();
        let location_args = location.into_any_arguments();
        let args = merge_any_arguments(mutation_args, location_args);
        let sqlx_query = sqlx::query_with(change_stmt, args);
        let result = sqlx_query.execute(self.get_pool()).await?;
        return Ok(result.rows_affected() as usize);
    }
}

#[cfg(test)]
mod tests {}
