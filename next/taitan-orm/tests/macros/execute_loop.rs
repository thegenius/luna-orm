use crate::entities::user::{User, UserMutation, UserPrimary, UserSelected, UserSelection};
use sqlx::Database;
use taitan_orm::database::sqlite::SqliteDatabase;
use taitan_orm::traits::Selection;
use taitan_orm::{ReaderApi, SqlExecutor, SqlGenericExecutor, TemplateApi, WriterApi, DB};
use taitan_orm_trait::{Entity, SelectedEntity};

// async fn test_insert_user<DB, EX, E, SL, SE>(db: &mut EX, user: &User) -> taitan_orm::Result<()>
// where
//     EX: SqlExecutor + SqlGenericExecutor + SqlGenericExecutor + WriterApi + ReaderApi + TemplateApi,
//     UserSelected: SelectedEntity<<EX as SqlGenericExecutor>::DB>,
//     // UserSelection: <UserSelected as SelectedEntity<<EX as SqlGenericExecutor>::DB>>::Selection
// {
//     let success = db.insert(user).await?;
//     assert!(success);
//
//     let mut selection = UserSelection::full_fields();
//     let primary = UserPrimary { id: user.id };
//     let entity_opt: Option<UserSelected> = db.select::<UserSelected, UserMutation>(&selection, &primary).await?;
//     assert!(entity_opt.is_some());
//
//     let selected_entity = entity_opt.unwrap();
//     assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
//     assert_eq!(selected_entity.name.unwrap(), user.name);
//     assert_eq!(selected_entity.age.unwrap(), user.age.unwrap());
//     assert_eq!(selected_entity.birthday.unwrap(), user.birthday.unwrap());
//     Ok(())
// }

async fn test_update_user(
    db: &mut SqliteDatabase,
    user_mutation: &UserMutation,
    user_primary: &UserPrimary,
) -> taitan_orm::Result<()> {
    let success = db.update(user_mutation, user_primary).await?;
    assert!(success);

    let selection = UserSelection::full_fields();
    let primary = UserPrimary {
        id: user_primary.id,
    };
    let entity_opt: Option<UserSelected> = db.select(&selection, &primary).await?;
    assert!(entity_opt.is_some());

    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id, user_mutation.request_id);
    assert_eq!(selected_entity.name, user_mutation.name);
    assert_eq!(selected_entity.age, user_mutation.age);
    assert_eq!(selected_entity.birthday, user_mutation.birthday);
    Ok(())
}

async fn test_upsert_user(db: &mut SqliteDatabase, user: &User) -> taitan_orm::Result<()> {
    let success = db.upsert(user).await?;
    assert!(success);

    let selection = UserSelection::full_fields();
    let user_primary: UserPrimary = UserPrimary { id: user.id };
    let entity_opt: Option<UserSelected> = db.select(&selection, &user_primary).await?;

    assert!(entity_opt.is_some());
    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
    assert_eq!(selected_entity.name.unwrap(), user.name);
    assert_eq!(selected_entity.age.unwrap(), user.age.unwrap());
    assert_eq!(selected_entity.birthday.unwrap(), user.birthday.unwrap());
    Ok(())
}
