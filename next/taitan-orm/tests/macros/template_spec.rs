use std::borrow::Cow;
use sqlx::sqlx_macros;
use time::PrimitiveDateTime;
use uuid::Uuid;
use taitan_orm::Schema;
use taitan_orm_macro::TemplateRecord;

#[derive(TemplateRecord, Clone, Debug)]
#[TemplateSql = "select * from #{name}"]
pub struct TestTemplate1<'a> {
   name: Cow<'a, str>,
}

#[derive(TemplateRecord, Clone, Debug)]
#[TemplateSql = "select * from #{name}"]
pub struct TestTemplate2 {
    name: String,
}

#[sqlx_macros::test]
pub async fn template_macro_spec() -> taitan_orm::Result<()> {




    Ok(())
}