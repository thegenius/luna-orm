pub struct SqliteLocalConfig {
    pub work_dir: String,
    pub db_file: String,
}

impl SqliteLocalConfig {
    pub fn new<S>(work_dir: S, db_file: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            work_dir: work_dir.into(),
            db_file: db_file.into(),
        }
    }
}