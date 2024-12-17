use sqlx::Arguments;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use crate::NotImplementError;

#[derive(Clone, Debug)]
pub struct Pagination {
    pub page_size: u64,
    pub page_num: u64,
}

impl Pagination {
    #[inline(always)]
    fn get_offset(&self) -> u64 {
        self.page_num * self.page_size
    }

    #[inline(always)]
    fn get_count(&self) -> u64 {
        self.page_size
    }

    fn gen_page_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        let offset: i64 = self.get_offset() as i64;
        let count: i64 = self.get_count() as i64;
        let mut arguments = SqliteArguments::default();
        arguments.add(offset)?;
        arguments.add(count)?;
        Ok(arguments)
    }
    fn gen_page_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        let offset = self.get_offset();
        let count = self.get_count();
        let mut arguments = MySqlArguments::default();
        arguments.add(offset)?;
        arguments.add(count)?;
        Ok(arguments)
    }
    fn gen_page_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        let offset: i64 = self.get_offset() as i64;
        let count: i64 = self.get_count() as i64;
        let mut arguments = PgArguments::default();
        arguments.add(offset)?;
        arguments.add(count)?;
        Ok(arguments)
    }
}