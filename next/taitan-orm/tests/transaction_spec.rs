use sqlx::sqlx_macros;

#[sqlx_macros::test]
pub async fn transaction_spec() -> taitan_orm::Result<()> {
    Ok(())
}