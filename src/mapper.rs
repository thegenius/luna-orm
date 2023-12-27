use async_trait::async_trait;
use luna_orm_trait::SqlxError;
use luna_orm_trait::{
    Entity, Location, Mutation, PageInfo, PagedList, Primary, SelectedEntity, Selection,
};
use sqlx::Any;

use sqlx::any::AnyRow;
use sqlx::Executor;

/*
#[async_trait]
pub trait GenericDaoMapper {
    async fn select<'e, EX, P, S, SE>(
        executor: EX,
        primary: P,
        selection: S,
    ) -> Result<Option<SE>, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        P: Primary + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
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
            sqlx::query_with(select_stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let entity_opt: Option<SE> = sqlx_query.fetch_optional(executor).await?;
        // let entity_opt: Option<Self::SE> = sqlx_query.fetch_optional(&mut *transation).await?;
        return Ok(entity_opt);
    }

    async fn create<'e, EX, E>(executor: EX, entity: E) -> Result<E, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        E: Entity + Send + Clone,
    {
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
            .execute(executor)
            .await?;
        return Ok(entity);
    }

    async fn insert<'e, EX, E>(executor: EX, entity: E) -> Result<bool, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        E: Entity + Send + Clone,
    {
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
            .execute(executor)
            .await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn upsert<'e, EX, E>(executor: EX, entity: E) -> Result<bool, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        E: Entity + Send + Clone,
    {
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
            .execute(executor)
            .await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn update<'e, EX, E>(executor: EX, entity: E) -> Result<bool, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        E: Entity + Send + Clone,
    {
        let table_name = entity.table_name('`');
        let body_assign_clause = entity.get_body_assignment_clause('`', "?");
        let primary_assign_clause = entity.get_primary_assignment_clause('`', "?");
        let update_stmt = &format!(
            "UPDATE {} SET {} WHERE {}",
            table_name, body_assign_clause, primary_assign_clause
        );
        let args = entity.any_arguments_of_update();
        let result = sqlx::query_with(update_stmt, args)
            .execute(executor)
            .await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn remove<'e, EX, P, E>(_executor: EX, _primary: P) -> Result<E, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        P: Primary + Send,
        E: Entity + Send + Clone,
    {
        todo!()
    }
    async fn delete<'e, EX, P>(executor: EX, primary: P) -> Result<bool, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        P: Primary + Send,
    {
        let table_name = primary.table_name('`');
        let where_clause = primary.get_where_clause('`', "?");
        let delete_stmt = &format!("DELETE FROM {} WHERE {}", table_name, where_clause);
        let args = primary.any_arguments();
        let result = sqlx::query_with(delete_stmt, args)
            .execute(executor)
            .await?;
        return Ok(result.rows_affected() > 0);
    }

    async fn search<'e, EX, L, S, SE>(
        executor: EX,
        location: L,
        selection: S,
    ) -> Result<Vec<SE>, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let table_name = location.table_name('`');
        let selected_fields = selection.get_sql_selection('`');
        let where_clause = location.get_where_clause('`', '?');
        let search_stmt = &format!(
            "SELECT {} FROM {} WHERE {}",
            selected_fields, table_name, where_clause
        );
        let args = location.any_arguments();
        let sqlx_query =
            sqlx::query_with(search_stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let entity_list = sqlx_query.fetch_all(executor).await?;
        return Ok(entity_list);
    }

    async fn search_paged<'e, EX, L, S, SE>(
        executor: EX,
        location: L,
        selection: S,
    ) -> Result<PagedList<SE>, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        L: Location + Send,
        S: Selection + Send,
        SE: SelectedEntity + Send + Unpin,
    {
        let table_name = location.table_name('`');
        let selected_fields = selection.get_sql_selection('`');
        let where_clause = location.get_where_clause('`', '?');
        let search_stmt = &format!(
            "SELECT {} FROM {} WHERE {}",
            selected_fields, table_name, where_clause
        );
        let args = location.any_arguments();
        let sqlx_query =
            sqlx::query_with(search_stmt, args).try_map(|row: AnyRow| SE::from_any_row(row));
        let entity_list = sqlx_query.fetch_all(executor).await?;
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

    async fn purify<'e, EX, L>(executor: EX, location: L) -> Result<usize, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        L: Location + Send,
    {
        let table_name = location.table_name('`');
        let where_clause = location.get_where_clause('`', '?');
        let delete_stmt = &format!("DELETE FROM {} WHERE {}", table_name, where_clause);
        let args = location.any_arguments();
        let sqlx_query = sqlx::query_with(delete_stmt, args);
        let result = sqlx_query.execute(executor).await?;
        return Ok(result.rows_affected() as usize);
    }

    async fn change<'e, EX, L, M>(
        executor: EX,
        location: L,
        mutation: M,
    ) -> Result<usize, SqlxError>
    where
        EX: 'e + Executor<'e, Database = Any>,
        L: Location + Send,
        M: Mutation + Send,
    {
        let table_name = location.table_name('`');
        let update_clause = mutation.get_update_clause('`', '?');
        let where_clause = location.get_where_clause('`', '?');
        let change_stmt = &format!(
            "UPDATE {} SET {} WHERE {}",
            table_name, update_clause, where_clause
        );
        let mutation_args = mutation.any_arguments();
        let location_args = location.any_arguments();
        let args = merge_any_arguments(mutation_args, location_args);
        let sqlx_query = sqlx::query_with(change_stmt, args);
        let result = sqlx_query.execute(executor).await?;
        return Ok(result.rows_affected() as usize);
    }
}

pub struct GenericDaoMapperImpl {}
impl GenericDaoMapper for GenericDaoMapperImpl {}
*/
