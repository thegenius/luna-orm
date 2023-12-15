use async_trait::async_trait;
use luna_orm_trait::{Entity, Location, Mutation, OrderBy, Pagination, Primary, Selection};

pub struct DefaultSqlGenerator {}
impl DefaultSqlGenerator {
    pub fn new() -> Self {
        Self {}
    }
}
impl SqlGenerator for DefaultSqlGenerator {}

#[async_trait]
pub trait SqlGenerator {
    // const WRAP_CHAR: char = '`'; can not made trait to trait object
    #[inline(always)]
    fn get_wrap_char(&self) -> char {
        '`'
    }

    // const PLACE_HOLDER: char = '?'; can not made trait to trait object
    #[inline(always)]
    fn get_place_holder(&self) -> char {
        '?'
    }

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

    #[inline(always)]
    fn post_process(&self, origin: String) -> String {
        origin
    }

    fn get_select_sql(&self, selection: &dyn Selection, primay: &dyn Primary) -> String {
        let table_name = primay.get_table_name();
        let selected_fields: Vec<String> = selection.get_selected_fields();
        let select_clause = wrap_fields(&selected_fields, self.get_wrap_char());
        let located_fields = primay.get_primary_field_names();
        let where_clause = wrap_locate_str_fields(
            located_fields,
            self.get_wrap_char(),
            self.get_place_holder(),
        );
        let select_sql = format!(
            "SELECT {} FROM {}{}{} WHERE {}",
            select_clause,
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            where_clause
        )
        .to_string();
        self.post_process(select_sql)
    }

    fn get_search_count_sql(&self, location: &dyn Location) -> String {
        let table_name = location.get_table_name();
        let where_clause = location.get_where_clause(self.get_wrap_char(), self.get_place_holder());

        let select_sql = format!(
            "SELECT COUNT(1) AS {}count{} FROM {}{}{} WHERE {}",
            self.get_wrap_char(),
            self.get_wrap_char(),
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            where_clause
        )
        .to_string();
        self.post_process(select_sql)
    }

    fn get_search_sql(
        &self,
        selection: &dyn Selection,
        location: &dyn Location,
        order_by: &dyn OrderBy,
    ) -> String {
        let selected_field_names = selection.get_selected_fields();
        let selected_fields = wrap_fields(&selected_field_names, self.get_wrap_char());
        let table_name = location.get_table_name();
        let where_clause = location.get_where_clause(self.get_wrap_char(), self.get_place_holder());
        let order_by_field_names = order_by.get_order_by_fields();
        if order_by_field_names.is_empty() {
            let select_sql = format!(
                "SELECT {} FROM {}{}{} WHERE {}",
                selected_fields,
                self.get_wrap_char(),
                table_name,
                self.get_wrap_char(),
                where_clause
            )
            .to_string();
            self.post_process(select_sql)
        } else {
            let order_by_fields = wrap_fields(&order_by_field_names, self.get_wrap_char());
            let select_sql = format!(
                "SELECT {} FROM {}{}{} WHERE {} ORDER BY {}",
                selected_fields,
                self.get_wrap_char(),
                table_name,
                self.get_wrap_char(),
                where_clause,
                order_by_fields
            )
            .to_string();
            self.post_process(select_sql)
        }
    }

    fn get_paged_search_sql(
        &self,
        selection: &dyn Selection,
        location: &dyn Location,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> String {
        let selected_field_names = selection.get_selected_fields();
        let selected_fields = wrap_fields(&selected_field_names, self.get_wrap_char());
        let table_name = location.get_table_name();
        let where_clause = location.get_where_clause(self.get_wrap_char(), self.get_place_holder());
        let offset = page.page_size * page.page_num;
        let count = page.page_size;
        let order_by_field_names = order_by.get_order_by_fields();
        if order_by_field_names.is_empty() {
            let select_sql = format!(
                "SELECT {} FROM {}{}{} WHERE {} LIMIT {},{}",
                selected_fields,
                self.get_wrap_char(),
                table_name,
                self.get_wrap_char(),
                where_clause,
                offset,
                count
            )
            .to_string();
            self.post_process(select_sql)
        } else {
            let order_by_fields = wrap_fields(&order_by_field_names, self.get_wrap_char());
            let select_sql = format!(
                "SELECT {} FROM {}{}{} WHERE {} ORDER BY {} LIMIT {},{}",
                selected_fields,
                self.get_wrap_char(),
                table_name,
                self.get_wrap_char(),
                where_clause,
                order_by_fields,
                offset,
                count
            )
            .to_string();
            self.post_process(select_sql)
        }
    }

    fn get_insert_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();
        let mut field_names = entity.get_primary_fields_name();
        field_names.extend(entity.get_body_fields_name());
        let fields = wrap_fields(&field_names, self.get_wrap_char());
        let marks = generate_question_mark_list(&field_names);
        let insert_sql = format!(
            "INSERT INTO {}{}{} ({}) VALUES({})",
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            fields,
            marks
        )
        .to_string();
        self.post_process(insert_sql)
    }

    fn get_upsert_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();
        let mut field_names = entity.get_primary_fields_name();
        field_names.extend(entity.get_body_fields_name());
        let fields = wrap_fields(&field_names, self.get_wrap_char());
        let primary_field_names = entity.get_primary_fields_name();
        let primary_fields = wrap_fields(&primary_field_names, self.get_wrap_char());
        let marks = generate_question_mark_list(&field_names);
        let body_field_names = entity.get_body_fields_name();
        let assign_clause = wrap_locate_fields(
            &body_field_names,
            self.get_wrap_char(),
            self.get_place_holder(),
        );

        let upsert_sql = format!(
            "INSERT INTO {}{}{} ({}) VALUES({})
            ON CONFLICT({}) DO UPDATE SET {}",
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            fields,
            marks,
            primary_fields,
            assign_clause
        )
        .to_string();
        self.post_process(upsert_sql)
    }

    fn get_update_sql(&self, mutation: &dyn Mutation, primary: &dyn Primary) -> String {
        let table_name = primary.get_table_name();
        let body_field_names = mutation.get_fields_name();
        let body_fields = wrap_locate_fields(
            &body_field_names,
            self.get_wrap_char(),
            self.get_place_holder(),
        );
        let primary_field_names = primary.get_primary_field_names();
        let primary_fields = wrap_locate_str_fields(
            &primary_field_names,
            self.get_wrap_char(),
            self.get_place_holder(),
        );
        let update_sql = format!(
            "UPDATE {}{}{} SET {} WHERE {}",
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            body_fields,
            primary_fields
        )
        .to_string();
        self.post_process(update_sql)
    }

    fn get_change_sql(&self, mutation: &dyn Mutation, location: &dyn Location) -> String {
        let table_name = location.get_table_name();
        let mutation_fields = mutation.get_fields_name();
        let update_clause = wrap_locate_fields(
            &mutation_fields,
            self.get_wrap_char(),
            self.get_place_holder(),
        );

        let where_clause = location.get_where_clause(self.get_wrap_char(), self.get_place_holder());
        let update_sql = format!(
            "UPDATE {}{}{} SET {} WHERE {}",
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            update_clause,
            where_clause
        )
        .to_string();
        self.post_process(update_sql)
    }

    fn get_delete_sql(&self, primary: &dyn Primary) -> String {
        let table_name = primary.get_table_name();
        let field_names = primary.get_primary_field_names();
        let where_clause =
            wrap_locate_str_fields(field_names, self.get_wrap_char(), self.get_place_holder());
        let delete_sql = format!(
            "DELETE FROM {}{}{} WHERE {}",
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            where_clause
        )
        .to_string();
        self.post_process(delete_sql)
    }

    fn get_purify_sql(&self, location: &dyn Location) -> String {
        let table_name = location.get_table_name();
        let where_clause = location.get_where_clause(self.get_wrap_char(), self.get_place_holder());
        let delete_sql = format!(
            "DELETE FROM {}{}{} WHERE {}",
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            where_clause
        )
        .to_string();
        self.post_process(delete_sql)
    }
}
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
