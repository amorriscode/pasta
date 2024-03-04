use sqlx::{query_as, Result, SqlitePool};

#[derive(Debug, Default, Clone)]
pub struct PasteboardItem {
    pub content: String,
}

pub async fn get_pasteboard_items(pool: &SqlitePool) -> Result<Vec<PasteboardItem>, sqlx::Error> {
    let result = query_as!(PasteboardItem, "SELECT content FROM pasteboard")
        .fetch_all(pool)
        .await?;

    Ok(result)
}

pub async fn get_latest_pasteboard_item(
    pool: &SqlitePool,
) -> Result<Option<PasteboardItem>, sqlx::Error> {
    let result = query_as!(
        PasteboardItem,
        "SELECT content FROM pasteboard ORDER BY created_at DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

pub async fn create_pasteboard_item(pool: &SqlitePool, content: String) -> Result<()> {
    sqlx::query!(r#"INSERT INTO pasteboard (content) VALUES (?1)"#, content)
        .execute(pool)
        .await?;

    Ok(())
}
