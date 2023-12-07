/*
use sqlx::{
    any::{AnyArguments, AnyQueryResult, AnyRow},
    Any, Executor,
};
*/

use async_trait::async_trait;
use luna_orm_trait::{Entity, Location, Primary, SelectedEntity, Selection, SqlxError};

#[inline]
fn wrap_fields(fields: &[String], wrap_char: char) -> String {
    fields
        .iter()
        .map(|e| format!("{}{}{}", wrap_char, e, wrap_char))
        .collect::<Vec<String>>()
        .join(",")
}

#[inline]
fn wrap_locate_fields(fields: &[String], wrap_char: char, place_holder: char) -> String {
    fields
        .iter()
        .map(|e| format!("{}{}{} = {}", wrap_char, e, wrap_char, place_holder))
        .collect::<Vec<String>>()
        .join(",")
}

#[inline]
fn wrap_str_fields(fields: &[&str], wrap_char: char) -> String {
    fields
        .iter()
        .map(|e| format!("{}{}{}", wrap_char, e, wrap_char))
        .collect::<Vec<String>>()
        .join(",")
}

#[inline]
fn wrap_locate_str_fields(fields: &[&str], wrap_char: char, place_holder: char) -> String {
    fields
        .iter()
        .map(|e| format!("{}{}{} = {}", wrap_char, e, wrap_char, place_holder))
        .collect::<Vec<String>>()
        .join(",")
}

#[inline]
fn wrap_pg_locate_str_fields(fields: &[&str], wrap_char: char) -> String {
    fields
        .iter()
        .enumerate()
        .map(|(i, e)| format!("{}{}{} = ${}", wrap_char, e, wrap_char, i + 1))
        .collect::<Vec<String>>()
        .join(",")
}

#[inline]
fn generate_question_marks(fields: &[&str]) -> String {
    fields
        .iter()
        .map(|_| "?".to_string())
        .collect::<Vec<String>>()
        .join(", ")
}
#[inline]
fn generate_question_mark_list(fields: &[String]) -> String {
    fields
        .iter()
        .map(|_| "?".to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

pub struct DefaultSqlGenerator {}
impl SqlGenerator for DefaultSqlGenerator {}

#[async_trait]
pub trait SqlGenerator {
    const WRAP_CHAR: char = '`';
    const PLACE_HOLDER: char = '?';

    #[inline]
    fn pg_post_process(&self, origin_sql: String) -> String {
        origin_sql
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '?' => format!("${}", i + 1),
                _ => c.to_string(),
            })
            .collect()
    }

    #[inline]
    fn post_process(&self, origin: String) -> String {
        origin
    }

    fn get_select_sql(&self, selection: &dyn Selection, primay: &dyn Primary) -> String {
        let table_name = primay.get_table_name();
        let selected_fields: Vec<String> = selection.get_selected_fields();
        let select_clause = wrap_fields(&selected_fields, Self::WRAP_CHAR);
        let located_fields = primay.get_primary_field_names();
        let where_clause =
            wrap_locate_str_fields(located_fields, Self::WRAP_CHAR, Self::PLACE_HOLDER);
        let select_sql = format!(
            "SELECT {} FROM {}{}{} WHERE {}",
            select_clause,
            Self::WRAP_CHAR,
            table_name,
            Self::WRAP_CHAR,
            where_clause
        )
        .to_string();
        self.post_process(select_sql)
    }

    fn get_search_sql(&self, selection: &dyn Selection, location: &dyn Location) -> String {
        let selected_field_names = selection.get_selected_fields();
        let selected_fields = wrap_fields(&selected_field_names, Self::WRAP_CHAR);
        let table_name = location.get_table_name();
        let where_clause = location.get_where_clause(Self::WRAP_CHAR, Self::PLACE_HOLDER);

        let select_sql = format!(
            "SELECT {} FROM {}{}{} WHERE {}",
            selected_fields,
            Self::WRAP_CHAR,
            table_name,
            Self::WRAP_CHAR,
            where_clause
        )
        .to_string();
        self.post_process(select_sql)
    }

    fn get_insert_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();
        let field_names = entity.get_fields_name();
        let fields = wrap_fields(&field_names, Self::WRAP_CHAR);
        let marks = generate_question_mark_list(&field_names);
        let insert_sql = format!(
            "INSERT INTO {}{}{} ({}) VALUES({})",
            Self::WRAP_CHAR,
            table_name,
            Self::WRAP_CHAR,
            fields,
            marks
        )
        .to_string();
        self.post_process(insert_sql)
    }

    fn get_upsert_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();
        let field_names = entity.get_fields_name();
        let fields = wrap_fields(&field_names, Self::WRAP_CHAR);
        let primary_field_names = entity.get_primary_fields_name();
        let primary_fields = wrap_fields(&primary_field_names, Self::WRAP_CHAR);
        let marks = generate_question_mark_list(&field_names);
        let body_field_names = entity.get_body_fields_name();
        let assign_clause =
            wrap_locate_fields(&body_field_names, Self::WRAP_CHAR, Self::PLACE_HOLDER);

        let upsert_sql = format!(
            "INSERT INTO {}{}{} ({}) VALUES({})
            ON CONFLICT({}) DO UPDATE SET {}",
            Self::WRAP_CHAR,
            table_name,
            Self::WRAP_CHAR,
            fields,
            marks,
            primary_fields,
            assign_clause
        )
        .to_string();
        self.post_process(upsert_sql)
    }

    fn get_update_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();
        let body_field_names = entity.get_body_fields_name();
        let body_fields =
            wrap_locate_fields(&body_field_names, Self::WRAP_CHAR, Self::PLACE_HOLDER);
        let primary_field_names = entity.get_primary_fields_name();
        let primary_fields =
            wrap_locate_fields(&primary_field_names, Self::WRAP_CHAR, Self::PLACE_HOLDER);
        let update_sql = format!(
            "UPDATE {}{}{} SET {} WHERE {}",
            Self::WRAP_CHAR,
            table_name,
            Self::WRAP_CHAR,
            body_fields,
            primary_fields
        )
        .to_string();
        self.post_process(update_sql)
    }

    fn get_delete_sql(&self, primary: &dyn Primary) -> String {
        let table_name = primary.get_table_name();
        let field_names = primary.get_primary_field_names();
        let where_clause = wrap_locate_str_fields(field_names, Self::WRAP_CHAR, Self::PLACE_HOLDER);
        let delete_sql = format!(
            "DELETE FROM {}{}{} WHERE {}",
            Self::WRAP_CHAR,
            table_name,
            Self::WRAP_CHAR,
            where_clause
        )
        .to_string();
        self.post_process(delete_sql)
    }

    /*
    async fn fetch_optional<'e, EX, SE>(
        &self,
        executor: EX,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<Option<SE>, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_opt: Option<SE> = query.fetch_optional(executor).await?;
        Ok(result_opt)
    }

    async fn fetch_all<'e, EX, SE>(
        &self,
        executor: EX,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<Vec<SE>, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        SE: SelectedEntity + Send + Unpin,
    {
        let query = sqlx::query_with(stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let result_vec: Vec<SE> = query.fetch_all(executor).await?;
        Ok(result_vec)
    }

    async fn execute<'e, EX>(
        &self,
        executor: EX,
        stmt: &str,
        args: AnyArguments<'_>,
    ) -> Result<AnyQueryResult, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
    {
        Ok(sqlx::query_with(stmt, args).execute(executor).await?)
    }
    */
}
