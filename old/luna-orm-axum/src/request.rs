use luna_orm::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "cmd")]
pub enum PostRequest<T>
where
    T: Entity + Send + Sync,
{
    #[serde(rename = "create")]
    Create { entity: T },
    #[serde(rename = "insert")]
    Insert { entity: T },
    #[serde(rename = "upsert")]
    Upsert { entity: T },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "cmd")]
pub enum PutRequest<M, P, L>
where
    M: Serialize + Mutation,
    P: Serialize + Primary,
    L: Serialize + Location,
{
    #[serde(rename = "update")]
    Update { mutation: M, primary: P },
    #[serde(rename = "change")]
    Change { mutation: M, location: L },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "cmd")]
pub enum DeleteRequest<P, L, S>
where
    P: Serialize + Primary,
    L: Serialize + Location,
    S: Serialize + Selection,
{
    #[serde(rename = "create")]
    Delete { primary: P },
    #[serde(rename = "insert")]
    Remove { primary: P, selection: S },
    #[serde(rename = "upsert")]
    Purify { location: L },
}

pub enum DynamicRequest<T, M, P, L, S>
where
    T: Entity + Send + Sync,
    M: Serialize + Mutation,
    P: Serialize + Primary,
    L: Serialize + Location,
    S: Serialize + Selection,
{
    Post(PostRequest<T>),
    Put(PutRequest<M, P, L>),
    Delete(DeleteRequest<P, L, S>),
}

#[cfg(test)]
mod test {
    use super::PostRequest;
    use luna_orm::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Schema, Serialize, Deserialize, PartialEq, Eq)]
    pub struct User {
        #[PrimaryKey]
        name: String,
    }

    #[test]
    fn test_insert_request() {
        let user = User {
            name: "test".to_string(),
        };
        let insert_req = PostRequest::Create { entity: user };
        let value = serde_yaml::to_string(&insert_req).unwrap();
        let expect_str = r#"cmd: create
entity:
  name: test
"#;
        assert_eq!(value, expect_str);
    }

    #[test]
    fn test_insert_request_deserialize() {
        let user = User {
            name: "test".to_string(),
        };
        let insert_req = PostRequest::Create { entity: user };
        let expect_str = r#"cmd: create
entity:
  name: test
"#;
        let value: PostRequest<User> = serde_yaml::from_str(expect_str).unwrap();
        assert_eq!(value, insert_req);
    }
}
