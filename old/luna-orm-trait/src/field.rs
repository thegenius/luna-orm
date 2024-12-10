use sqlx::database::Database;
use sqlx::error::BoxDynError;
use sqlx::Any;
use sqlx::ColumnIndex;
use sqlx::Decode;
use sqlx::Error;
use sqlx::Type;
use sqlx::ValueRef;

pub trait Decodable<'r, DB: Database>: Decode<'r, DB> + Type<DB> {}
impl<'r, T, DB: Database> Decodable<'r, DB> for T where T: Decode<'r, DB> + Type<DB> {}

pub enum Field<T>
where
    for<'r> T: Decodable<'r, Any>,
{
    Selected(Option<bool>),
    Value(Option<T>),
}

impl<T> Decode<'_, Any> for Field<T>
where
    for <'r> T: Decodable<'r, Any>,
{
    #[inline]
    fn decode(value: <Any as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
        let val = T::decode(value)?;
        Ok(Self::Value(Some(val)))
    }
}

impl<T> Field<T>
where
    for<'r> T: Decode<'r, Any> + Type<Any>,
{
    pub fn value(val: T) -> Self {
        Self::Value(Some(val))
    }

    pub fn selected() -> Self {
        Self::Selected(Some(true))
    }

    pub fn is_selected(&self) -> bool {
        match &self {
            Self::Selected(s) => s.unwrap_or(false),
            Self::Value(v) => v.is_some(),
        }
    }

    pub fn unbox(self) -> Option<T> {
        match self {
            Self::Selected(_) => None,
            Self::Value(v) => v,
        }
    }

    pub fn get(&self) -> Option<&T> {
        match &self {
            Self::Selected(_) => None,
            Self::Value(v) => v.as_ref(),
        }
    }

    /*
    pub fn try_get<'r, I>(&'r self, index: I) -> Result<T, Error>
    where
        I: ColumnIndex<T>,
    {
        todo!()
    }
    */
}

#[cfg(test)]
mod test {
    use super::Field;
    use sqlx::Any;

    struct User {
        name: Field<String>,
        age: Field<i32>,
    }
}
