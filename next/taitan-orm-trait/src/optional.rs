use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum Optional<T> {
    None,    // 不传递到数据库层
    Null,    // 传递到数据库，值为null
    Some(T), // 传递到数据库，值为具体值
}

impl<T> PartialEq<Option<T>> for Optional<T> {
    fn eq(&self, other: &Option<T>) -> bool {
        match self {
            Optional::Null => false,
            Optional::None => matches!(other, None),
            Optional::Some(s) => matches!(other, Some(s)),
        }
    }
}

impl<T> Default for Optional<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => Optional::Some(value),
            None => Optional::None,
        }
    }
}

impl<T> Optional<T> {
    pub fn unwrap(self) -> T {
        match self {
            Optional::Some(value) => value,
            _ => panic!("called `Optional::unwrap()` on a `None`"),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Optional::Some(value) => value,
            _ => default,
        }
    }

    pub fn unwrap_or_default(self) -> T where T: Default {
        match self {
            Optional::Some(value) => value,
            _ => Default::default(),
        }
    }

    pub fn not_some(&self) -> bool {
        match self {
            Optional::Some(_) => false,
            _ => true,
        }
    }

    pub fn not_none(&self) -> bool {
        match self {
            Optional::None => false,
            _ => true,
        }
    }

    pub fn not_null(&self) -> bool {
        match self {
            Optional::Null => false,
            _ => true,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Optional::Some(_) => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Optional::None => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Optional::Null => true,
            _ => false,
        }
    }
}
