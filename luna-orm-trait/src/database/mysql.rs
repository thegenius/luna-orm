use crate::database::Database;
use crate::database::SqlType;
use sqlx::MySql;

impl Database for MySql {
    const WRAP_CHAR: char = '`';
    const PLACE_HOLDER: char = '?';
}
