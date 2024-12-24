use sqlx::sqlx_macros;
use std::borrow::Cow;
use taitan_orm::Schema;
use taitan_orm_macro::TemplateRecord;
use taitan_orm_trait::TemplateRecord;
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from ${name}"]
pub struct TestTemplate1<'a> {
    name: Cow<'a, str>,
}

// #[derive(TemplateRecord, Clone, Debug)]
// #[TemplateSql = "select * from ${name}"]
// pub struct TestTemplate1Template<'a> {
//     name: Cow<'a, str>,
// }
// impl<'a> From<TestTemplate1<'a>> for TestTemplate1Template<'a> {
//     fn from(orig: TestTemplate1<'a>) -> TestTemplate1Template<'a> {
//         Self {
//             name: orig.name.clone(),
//         }
//     }
// }

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from #{name}"]
pub struct TestTemplate2 {
    name: String,
}

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from ${name} #{age}"]
pub struct TestTemplate3<'a> {
    name: Cow<'a, str>,
    age: i32,
}

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from ${name} #{age} 'hello' "]
pub struct TestTemplate4<'a> {
    name: Cow<'a, str>,
    age: i32,
}

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from ${name} #{age} \"hello ${name}\""]
#[count_sql = "select count(*) from ${name} #{age} \"hello ${name}\""]
pub struct TestTemplate5<'a> {
    name: Cow<'a, str>,
    age: i32,
}

#[sqlx_macros::test]
pub async fn template_macro_spec() -> taitan_orm::Result<()> {
    let template = TestTemplate1 {
        name: Cow::Borrowed("wang"),
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from wang");

    let template = TestTemplate2 {
        name: String::from("wang"),
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from ?");

    let template = TestTemplate3 {
        name: Cow::Borrowed("wang"),
        age: 23,
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from wang ?");

    let template = TestTemplate4 {
        name: Cow::Borrowed("wang"),
        age: 23,
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from wang ? 'hello'");

    let template = TestTemplate5 {
        name: Cow::Borrowed("wang"),
        age: 23,
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from wang ? \"hello ${name}\"");

    Ok(())
}
