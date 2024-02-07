use crate::Integer;
use num_traits::PrimInt;
use sqlx::any::AnyValue;
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;
use sqlx::sqlite::SqliteArgumentValue;
use sqlx::Database;
use sqlx::{Any, Sqlite};
use sqlx::{Encode, Type};
use sqlx_core::any::AnyTypeInfo;
use sqlx_core::any::AnyValueKind;

impl<'a> Encode<'a, Sqlite> for Integer<i32> {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        buf.push(SqliteArgumentValue::Int(self.0));
        return IsNull::No;
    }
}
impl<'a> Encode<'a, Sqlite> for Integer<i64> {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        buf.push(SqliteArgumentValue::Int64(self.0));
        return IsNull::No;
    }
}

impl Type<Sqlite> for Integer<i32> {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <i32 as Type<Sqlite>>::type_info()
    }
}

impl Type<Sqlite> for Integer<i64> {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
}

/******************************************/
impl<'a> Encode<'a, Any> for Integer<i32> {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        buf.0.push(AnyValueKind::Integer(self.0));
        return IsNull::No;
    }
}
impl<'a> Encode<'a, Any> for Integer<i64> {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        buf.0.push(AnyValueKind::BigInt(self.0));
        return IsNull::No;
    }
}

impl Type<Any> for Integer<i32> {
    fn type_info() -> <Any as Database>::TypeInfo {
        <i32 as Type<Any>>::type_info()
    }
}

impl Type<Any> for Integer<i64> {
    fn type_info() -> <Any as Database>::TypeInfo {
        <i64 as Type<Any>>::type_info()
    }
}
