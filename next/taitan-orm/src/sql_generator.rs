use taitan_orm_trait::FromClause;
use taitan_orm_trait::JoinedConditions;
use taitan_orm_trait::{Entity, Location, Mutation, Primary};
use taitan_orm_trait::{OrderBy, Selection};
use taitan_orm_trait::pagination::Pagination;

#[derive(Default, Debug, Clone)]
pub struct DefaultSqlGenerator {}
impl DefaultSqlGenerator {
    pub fn new() -> Self {
        Self {}
    }
}
impl SqlGenerator for DefaultSqlGenerator {}

#[derive(Default, Debug, Clone)]
pub struct MySqlGenerator {}
impl MySqlGenerator {
    pub fn new() -> Self {
        Self {}
    }
}
impl SqlGenerator for MySqlGenerator {
    fn get_upsert_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();
        let field_names = entity.get_insert_fields();
        let fields = wrap_fields(&field_names, self.get_wrap_char());
        let marks = generate_question_mark_list(&field_names);
        let set_field_names = entity.get_upsert_set_fields();
        let assign_clause = wrap_locate_fields(
            &set_field_names,
            self.get_wrap_char(),
            self.get_place_holder(),
        );

        let upsert_sql = format!(
            "INSERT INTO {}{}{} ({}) VALUES({})
            ON DUPLICATE KEY UPDATE SET {}",
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            fields,
            marks,
            assign_clause
        )
        .to_string();
        self.post_process(upsert_sql)
    }

    fn get_create_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();
        let field_names = entity.get_insert_fields();
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
}

#[derive(Default, Debug, Clone)]
pub struct PostgresGenerator {}
impl PostgresGenerator {
    pub fn new() -> Self {
        Self {}
    }
}
impl SqlGenerator for PostgresGenerator {
    fn post_process(&self, origin: String) -> String {
        self.pg_post_process(origin)
    }
}

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

    fn get_last_row_id_sql(&self) -> &'static str {
        "SELECT last_insert_rowid() as `last_row_id`"
    }

    fn get_select_sql(&self, selection: &dyn Selection, primary: &dyn Primary) -> String {
        let table_name = primary.get_table_name();
        let selected_fields: Vec<String> = selection.get_selected_fields();
        let select_clause = wrap_fields(&selected_fields, self.get_wrap_char());
        let located_fields = primary.get_primary_field_names();
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

    fn get_search_all_sql(&self, selection: &dyn Selection) -> String {
        let table_name = selection.get_table_name();
        let selected_field_names = selection.get_selected_fields();
        let selected_fields = wrap_fields(&selected_field_names, self.get_wrap_char());
        let select_sql = format!(
            "SELECT {} FROM {}{}{}",
            selected_fields,
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
        )
        .to_string();
        self.post_process(select_sql)
    }

    fn get_search_sql(
        &self,
        selection: &dyn Selection,
        location: &dyn Location,
        order_by: Option<&dyn OrderBy>,
    ) -> String {
        let selected_field_names = selection.get_selected_fields();
        let selected_fields = wrap_fields(&selected_field_names, self.get_wrap_char());
        let table_name = location.get_table_name();
        let where_clause = location.get_where_clause(self.get_wrap_char(), self.get_place_holder());
        if order_by.is_none() {
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
            let order_by_field_names = order_by.unwrap().get_order_by_fields();
            let order_by_fields = wrap_str_fields(&order_by_field_names, self.get_wrap_char());
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

    fn get_limit_sql(&self, page: &Pagination) -> String {
        let offset = page.page_size * page.page_num;
        let count = page.page_size;
        return format!("{}, {}", offset, count);
    }

    fn get_paged_search_sql(
        &self,
        selection: &dyn Selection,
        location: &dyn Location,
        order_by: Option<&dyn OrderBy>,
        page: &Pagination,
    ) -> String {
        let selected_field_names = selection.get_selected_fields();
        let selected_fields = wrap_fields(&selected_field_names, self.get_wrap_char());
        let table_name = location.get_table_name();
        let where_clause = location.get_where_clause(self.get_wrap_char(), self.get_place_holder());
        let offset = page.page_size * page.page_num;
        let count = page.page_size;
        if order_by.is_none() {
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
            let order_by_field_names = order_by.unwrap().get_order_by_fields();
            let order_by_fields = wrap_str_fields(order_by_field_names, self.get_wrap_char());
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

    fn get_page_joined_search_sql(
        &self,
        joined_conds: &JoinedConditions,
        locations: &Vec<&dyn Location>,
        _order_by: Option<&dyn OrderBy>,
        selections: &Vec<&dyn Selection>,
        _page: &Pagination,
    ) -> String {
        let mut selected_field_names: Vec<String> = Vec::new();
        for selection in selections {
            let fields = selection.get_selected_fields();
            selected_field_names.extend(fields);
        }
        let selected_fields = wrap_fields(&selected_field_names, self.get_wrap_char());

        let mut location_stmts: Vec<String> = Vec::new();
        for location in locations {
            let where_clause =
                location.get_where_clause(self.get_wrap_char(), self.get_place_holder());
            location_stmts.push(where_clause);
        }
        let where_clause = location_stmts.join(",");
        let from_clause = joined_conds.get_from_clause();
        let sql: String = format!(
            "SELECT {} FROM {} WHERE {}",
            selected_fields, from_clause, where_clause
        )
        .to_string();
        self.post_process(sql)
    }

    fn get_insert_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();
        let field_names = entity.get_insert_fields();
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

    fn get_create_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();
        let field_names = entity.get_insert_fields();
        let fields = wrap_fields(&field_names, self.get_wrap_char());
        let marks = generate_question_mark_list(&field_names);
        let auto_field_name = entity.get_auto_increment_field();
        let create_sql = if auto_field_name.is_some() {
            let auto_field_name = auto_field_name.unwrap();
            format!(
                "INSERT INTO {}{}{} ({}) VALUES({}) RETURNING {}{}{} AS last_row_id",
                self.get_wrap_char(),
                table_name,
                self.get_wrap_char(),
                fields,
                marks,
                self.get_wrap_char(),
                auto_field_name,
                self.get_wrap_char()
            )
            .to_string()
        } else {
            format!(
                "INSERT INTO {}{}{} ({}) VALUES({})",
                self.get_wrap_char(),
                table_name,
                self.get_wrap_char(),
                fields,
                marks
            )
            .to_string()
        };
        self.post_process(create_sql)
    }

    fn get_upsert_sql(&self, entity: &dyn Entity) -> String {
        let table_name = entity.get_table_name();

        let field_names = entity.get_insert_fields();
        let fields = wrap_fields(&field_names, self.get_wrap_char());
        let marks = generate_question_mark_list(&field_names);
        let set_field_names = entity.get_upsert_set_fields();
        let assign_clause = wrap_locate_fields(
            &set_field_names,
            self.get_wrap_char(),
            self.get_place_holder(),
        );

        let upsert_sql = format!(
            "INSERT INTO {}{}{} ({}) VALUES({})
            ON CONFLICT DO UPDATE SET {}",
            self.get_wrap_char(),
            table_name,
            self.get_wrap_char(),
            fields,
            marks,
            assign_clause
        )
        .to_string();
        self.post_process(upsert_sql)
    }

    fn get_update_sql<M: Mutation>(&self, mutation: &M, primary: &M::Primary) -> String {
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

    fn get_change_sql<M: Mutation>(&self, mutation: &M, location: &M::Location) -> String {
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
