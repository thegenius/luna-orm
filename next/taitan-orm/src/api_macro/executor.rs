#[macro_export]
macro_rules! fetch_execute_option_fn {
    ($executor:expr) => {
        let ex = $executor;
        let query: Query<'a, Self::DB, SqliteArguments<'a>> = query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row_full(result)?))
        } else {
            Ok(None)
        }
    };
}
