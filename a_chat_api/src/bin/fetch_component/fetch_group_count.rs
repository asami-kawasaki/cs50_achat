use axum::{
    response::Json,
    extract::{Path},
};
// シリアライズ: RustのオブジェクトをJSON形式に変換
// デシリアライズ : JSON形式をRustのオブジェクトに変換
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use sqlx::mysql::MySqlPool;
use std::{env, fmt::Debug};
/*
  ユーザーの所属するグループ数取得
*/
// handler
#[derive(Debug, Deserialize, Serialize)]
pub struct FetchGroupCountPath {
    user_id: String,
}
pub async fn handler_fetch_group_count(Path(path): Path<FetchGroupCountPath>) -> Json<Value> {
    let user_id = path.user_id;

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let group_count = fetch_group_count(&pool, &user_id).await.unwrap();
    Json(json!({ "group_count": group_count }))
}

// SQL実行部分
pub async fn fetch_group_count(pool: &MySqlPool, user_id:&str) -> anyhow::Result<i64>{
    let group_count = sqlx::query!(
        r#"
            SELECT
                COUNT(*) as group_count
            FROM
                group_member as gm
                INNER JOIN
                    group_chat_room as g
                ON  gm.group_chat_room_id = g.id
            WHERE
                gm.user_id = ?
            AND gm.leave_flag = FALSE
            AND g.delete_flag = FALSE
            "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    Ok(group_count[0].group_count)
}