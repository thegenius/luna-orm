use crate::{FieldType, Integer};
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

/******************************************/
impl<'a> Encode<'a, Any> for FieldType<'a> {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        match self {
            Self::SmallInt(val) => {
                buf.0.push(AnyValueKind::SmallInt(val.0));
            }
            Self::Int(val) => {
                buf.0.push(AnyValueKind::Integer(val.0));
            }
            Self::BigInt(val) => {
                buf.0.push(AnyValueKind::BigInt(val.0));
            }
            Self::SmallUInt(val) => {
                buf.0.push(AnyValueKind::SmallInt(val.0 as i16));
            }
            Self::UInt(val) => {
                buf.0.push(AnyValueKind::Integer(val.0 as i32));
            }
            Self::BigUInt(val) => {
                buf.0.push(AnyValueKind::BigInt(val.0 as i64));
            }
            Self::Text(val) => {
                buf.0.push(AnyValueKind::Text(val.0.clone()));
            }
        }
        return IsNull::No;
    }
}
