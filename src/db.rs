use sqlx::{query_as, Result, SqlitePool};

#[derive(Debug, Default, Clone)]
pub struct ClipboardItem {
    pub content: String,
}

pub async fn get_clipboard_items(pool: &SqlitePool) -> Result<Vec<ClipboardItem>, sqlx::Error> {
    let result = query_as!(ClipboardItem, "SELECT content FROM clipboard")
        .fetch_all(pool)
        .await?;

    Ok(result)
}

pub async fn get_latest_clipboard_item(
    pool: &SqlitePool,
) -> Result<Option<ClipboardItem>, sqlx::Error> {
    let result = query_as!(
        ClipboardItem,
        "SELECT content FROM clipboard ORDER BY created_at DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

pub async fn create_clipboard_item(pool: &SqlitePool, content: String) -> Result<()> {
    sqlx::query!(r#"INSERT INTO clipboard (content) VALUES (?1)"#, content)
        .execute(pool)
        .await?;

    Ok(())
}
