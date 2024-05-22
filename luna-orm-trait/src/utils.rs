
use sqlx::mysql::MySqlArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx::any::AnyArguments;
use sqlx::{Any, MySql, Sqlite};
use sqlx::{Encode, Type};
use sqlx::Arguments;

pub fn add_val_to_any_arguments<'q, T>(args: &mut AnyArguments<'q>, value: &T)
where
    T: 'q + Send + Encode<'q, Any> + Type<Any>,
{
    let _ = value.encode_by_ref(&mut args.values);
}

pub fn add_val_to_mysql_arguments<'q, T>(args: &mut MySqlArguments, value: &'q T)
where
    T: 'q + Sync + Encode<'q, MySql> + Type<MySql>,
{
    args.add(value)
}

pub fn add_val_to_sqlite_arguments<'q, T>(args: &'q mut SqliteArguments<'q>, value: &'q T)
where
    T: 'q + Sync + Encode<'q, Sqlite> + Type<Sqlite>,
{
    args.add(value)
}







pub trait ArrayStrEqual {
    fn equal<S>(&self, arr: &[S]) -> bool
    where
        S: AsRef<str>;
}

impl<const N: usize> ArrayStrEqual for [&str; N] {
    fn equal<S>(&self, arr: &[S]) -> bool
    where
        S: AsRef<str>,
    {
        let n = self.len();
        if n != arr.len() {
            return false;
        }
        for i in 0..n {
            let a = self[i];
            let b = &arr[i];
            if a != b.as_ref() {
                return false;
            }
        }
        true
    }
}

pub trait ArrayEqual<T>
where
    T: PartialEq,
{
    fn equal<S>(&self, arr: &[S]) -> bool
    where
        S: AsRef<T>;
}

impl<const N: usize, T: PartialEq> ArrayEqual<T> for [T; N] {
    fn equal<S>(&self, arr: &[S]) -> bool
    where
        S: AsRef<T>,
    {
        let n = self.len();
        if n != arr.len() {
            return false;
        }
        for i in 0..n {
            let a = &self[i];
            let b = arr[i].as_ref();
            if a != b {
                return false;
            }
        }
        true
    }
}

pub fn array_str_equal(arr_str: &[&str], arr_string: &[&str]) -> bool {
    arr_str.iter().all(|e| arr_string.contains(e))
}
