use sqlx::sqlx_macros;
use std::borrow::Cow;
use taitan_orm::Schema;
use taitan_orm_macro::TemplateRecord;
use time::PrimitiveDateTime;
use uuid::Uuid;


#[derive(TemplateRecord, Clone, Debug)]
#[TemplateSql = "select * from ${name}"]
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
#[TemplateSql = "select * from #{name}"]
pub struct TestTemplate2 {
    name: String,
}

#[derive(TemplateRecord, Clone, Debug)]
#[TemplateSql = "select * from ${name} #{age}"]
pub struct TestTemplate3<'a> {
    name: Cow<'a, str>,
    age: i32
}

#[sqlx_macros::test]
pub async fn template_macro_spec() -> taitan_orm::Result<()> {
    Ok(())
}
