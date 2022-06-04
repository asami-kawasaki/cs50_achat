use axum::{
    routing::get,
    routing::post,
    Router,
    response::Json,
    extract::{Path,Query},
};
mod component;
// シリアライズ: RustのオブジェクトをJSON形式に変換
// デシリアライズ : JSON形式をRustのオブジェクトに変換
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use sqlx::mysql::MySqlPool;
use std::env;
use dotenv::dotenv;
use std::time::SystemTime;

#[tokio::main]
async fn main(){
    // .endファイルの中身の変数を取得し、環境変数として使用できるようにする
    dotenv().ok();
    // 外部モジュールの呼び出し(2015年版)
    // component::header::headers();
    //単一ルートでアプリケーションを構築する
    // handler: 何らかの処理要求が発生した時に起動されるプログラムのこと
    // handlerはアプリケーションのロジックが存在する場所
    let app = Router::new()
        .route("/api/getAllUsers", get(handler_fetch_all_users))
        .route("/api/signup", post(handler_sign_up))
        .route("/api/signup/isAvailableUserIdValidation/:user_id", get(handler_is_available_user_id_validation))
        .route("/api/login", post(handler_log_in))
        .route("/api/users/:user_id/home", get(handler_search_name))
        .route("/api/users/:user_id/groups", get(handler_fetch_group_list))
        .route("/api/users/:user_id/groups/leave", post(handler_leave_group))
        .route("/api/users/:user_id/groups/add", post(handler_add_group))
        .route("/api/users/:user_id/group-count", get(handler_fetch_group_count))
        .route("/api/users/:user_id/friend-count", get(handler_fetch_friend_count))
        .route("/api/users/:user_id/friends", get(handler_fetch_friend_list))
        .route("/api/users/:user_id/friends", post(handler_add_friend))
        .route("/api/users/:user_id/profile", get(handler_fetch_profile_by_user_id))
        .route("/api/users/:user_id/profile", post(handler_update_profile))
        .route("/api/users/:user_id/user", get(handler_fetch_friend_info_by_friend_user_id));

    // localhost:3000 で hyper と共に実行する
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
        
}

/*
  全ユーザ情報を取得
*/
// handler
async fn handler_fetch_all_users() -> Json<Value> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let users = fetch_all_users(&pool).await.unwrap();
    Json(json!({ "users": users }))
}

// SQL実行部分
use a_chat_api::models::User;
async fn fetch_all_users(pool: &MySqlPool) -> anyhow::Result<Vec<User>> {
    let users = sqlx::query!(
        r#"
            SELECT *
            FROM user
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let mut result:Vec<User> = vec![];

    for row in &users {
        let user = User {
            id: row.id.to_string(),
            nickname:  match &row.nickname {
                Some(nickname) => Some(nickname.to_string()),
                None => None
            },
            mail: row.mail.to_string(),
            password: row.password.to_string(),
            profile_image:  match &row.profile_image {
                Some(profile_image) => Some(profile_image.to_string()),
                None => None
            },
            delete_flag: if row.delete_flag == 1 {true} else {false},
            search_flag: if row.search_flag == 1 {true} else {false},
            created_at: row.created_at,
            updated_at: row.updated_at
          };
          result.push(user);
    }
    Ok(result)
}

/*
  会員登録
*/
// handler
async fn handler_sign_up(body_json: Json<Value>) -> Json<Value> {
    // user_idの取得
    let user_id = body_json.0.get("user_id")
    .unwrap()
    .as_str()
    .unwrap();

    // mailの取得
    let mail = body_json.0.get("mail")
    .unwrap()
    .as_str()
    .unwrap();

    // passwordの取得
    let password = body_json.0.get("password")
    .unwrap()
    .as_str()
    .unwrap();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    sign_up(&pool, user_id, mail, password).await.unwrap();

    Json(json!({ "user_id": user_id }))
}

// SQL実行部分
async fn sign_up(pool: &MySqlPool, user_id:&str, mail: &str, password: &str) -> anyhow::Result<()> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    sqlx::query!(
        r#"
INSERT INTO user ( id, mail, password, created_at )
VALUES ( ?, ? , ? , ? )
        "#,
        user_id,
        mail,
        password,
        now
    )
    .execute(pool)
    .await
    .unwrap();
    
    Ok(())
}

/*
  登録するユーザーIDが使用可能かどうかチェック
*/
// handler
#[derive(Debug, Deserialize, Serialize)]
struct IsAvailableUserIdValidationPath {
    user_id: String,
}
async fn handler_is_available_user_id_validation(Path(path): Path<IsAvailableUserIdValidationPath>) -> Json<Value> {
    let user_id = path.user_id;

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let is_available_user_id_validation = is_available_user_id_validation(&pool, &user_id).await.unwrap();
    Json(json!({ "is_available_user_id_validation": is_available_user_id_validation }))
}

// SQL実行部分
async fn is_available_user_id_validation(pool: &MySqlPool, user_id:&str) -> anyhow::Result<bool>{
    let user = sqlx::query!(
        r#"
            SELECT *
            FROM user
            WHERE id = ?
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let result;

    if user.len() == 0 {
        // まだ該当user_idは使用されていない
        result = true
    } else {
        // 既に該当user_idは使用されている
        result = false
    }
    Ok(result)
}

/*
  ログイン
*/
#[derive(Debug, Deserialize, Serialize)]
struct LoginResult {
    user_id: Option<String>,
    certification_result: bool
}

// handler
async fn handler_log_in(body_json: Json<Value>) -> Json<Value> {
    // mailの取得
    let mail = body_json.0.get("mail")
    .unwrap()
    .as_str()
    .unwrap();

    // passwordの取得
    let password = body_json.0.get("password")
    .unwrap()
    .as_str()
    .unwrap();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let result = log_in(&pool, &mail, &password).await.unwrap();
    Json(json!({ "user_id": result.user_id, "certification_result": result.certification_result }))
}

// SQL実行部分
async fn log_in(pool: &MySqlPool, mail: &str, password: &str ) -> anyhow::Result<LoginResult> {
    let user = sqlx::query!(
        r#"
            SELECT *
            FROM user
            WHERE mail = ? AND password = ?
        "#,
        mail,
        password
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let result;

    if user.len() == 0 {
        // まだ会員登録されていない
        result = LoginResult {
            user_id : None ,
            certification_result: false
        }
    } else {
        // 既に会員登録されている
        result = LoginResult {
            user_id : Some(user[0].id.to_string()) ,
            certification_result: true
        }
    }
    Ok(result)
}

/*
  ニックネームまたはグループ名の検索でヒットするユーザーまたはグループ情報の取得
*/
#[derive(Debug, Deserialize, Serialize)]
struct SearchNamePath {
    user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SearchNameQuery {
    search_text: String,
}

// デフォルト値の取得
impl Default for SearchNameQuery {
    fn default() -> Self {
        Self { search_text: String::from("") }
    }
}

// handler
async fn handler_search_name(
    Path(path): Path<SearchNamePath>,
    search_name_query: Option<Query<SearchNameQuery>>,
) -> Json<Value> {
    let user_id = path.user_id;
    // unwrap_or_default: Okの場合値を返し、Errの場合値の型のデフォルトを返す
    let Query(search_name_query) = search_name_query.unwrap_or_default();
    let search_text = search_name_query.search_text;

    // friends
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let friends = search_name_friends(&pool, &user_id, &search_text).await.unwrap();
    
    // groups
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let groups = search_name_groups(&pool, &user_id, &search_text).await.unwrap();
    Json(json!({ "friends": friends, "groups": groups  }))
}

#[derive(Debug, Deserialize, Serialize)]
enum SearchNameFriendListResultEnum {
    SearchNameFriendListNoneResult {
        direct_chat_room_id: Option<u64>,
        friend_use_id: Option<String>,
        friend_profile_image: Option<String>,
        friend_nickname: Option<String>
    },
    SearchNameFriendListResult {
        direct_chat_room_id: u64,
        friend_use_id: String,
        friend_profile_image: Option<String>,
        friend_nickname: Option<String>
    }
}

// SQL実行部分(Friends)
async fn search_name_friends(pool: &MySqlPool, user_id: &str, search_text: &str) -> anyhow::Result<Vec<SearchNameFriendListResultEnum>> {
    // 取得したいデータ
    // "direct_chat_room_id": "1",
    // "friend_use_id": "asami111",
    // "friend_profile_image": null,
    // "friend_nickname": "検索結果name"
    
    // 条件
    // ①userテーブルのnicknameが一致
    // ②followテーブルのfrom_user_idが自分の場合のto_user_idが友達のuser_id
    // ②direct_memberテーブルのdeleteフラグがfalseである(非表示の友達も表示する)
    
    let search_name_friend_list = sqlx::query!(
        r#"
            SELECT
                u.id as friend_user_id,
                u.nickname as friend_nickname,
                u.profile_image as friend_profile_image,
                f.direct_chat_room_id as direct_chat_room_id
            FROM
                user as u
                INNER JOIN
                    follow as f
                ON  u.id = f.to_user_id
            WHERE
                u.id IN(
                    SELECT
                        id
                    FROM
                        user
                    WHERE
                        nickname = ?
                )
            AND u.id IN(
                    SELECT
                        f.to_user_id
                    FROM
                        follow as f
                    WHERE
                        f.from_user_id = ?
                )
            AND f.direct_chat_room_id IN(
                    SELECT
                        direct_chat_room_id
                    FROM
                        direct_member
                    WHERE
                        user_id = ?
                )
        "#,
        search_text,
        user_id,
        user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let mut result:Vec<SearchNameFriendListResultEnum> = vec![];

    if search_name_friend_list.len() == 0 {
        result.push(
            SearchNameFriendListResultEnum::SearchNameFriendListNoneResult {
                direct_chat_room_id: None,
                friend_use_id: None,
                friend_profile_image: None,
                friend_nickname: None,
            }
        )
    } else {
        let mut result:Vec<SearchNameFriendListResultEnum> = vec![];
        for row in &search_name_friend_list {
            let friend = SearchNameFriendListResultEnum::SearchNameFriendListResult {
                direct_chat_room_id: row.direct_chat_room_id, 
                friend_use_id: row.friend_user_id.to_string(),
                friend_profile_image:  match &row.friend_profile_image {
                    Some(friend_profile_image) => Some(friend_profile_image.to_string()),
                    None => None
                },
                friend_nickname:  match &row.friend_nickname {
                    Some(friend_nickname) => Some(friend_nickname.to_string()),
                    None => None
                },
              };
              result.push(friend);
        }
    }

    Ok(result)
}

#[derive(Debug, Deserialize, Serialize)]
struct SearchNameGroupListResult {
    group_chat_room_id: String,
    group_name: String,
    group_image: Option<String>
}

// SQL実行部分(Groups)
async fn search_name_groups(pool: &MySqlPool, user_id: &str, search_text: &str) -> anyhow::Result<Vec<SearchNameGroupListResult>> {
    // 取得したいデータ
    // "group_chat_room_id": "12",
    // "group_name": "検索結果グループ",
    // "group_image": null)
    
    // 条件
    // ①group_memberテーブルのuser_idが自分のuser_idであるかつdelete_flagとhidden_flagがfalseであるgroup_chat_room_id
    // ②①の取得結果のgroup_chat_room_idのうち、group_nameが一致
    let search_name_group_list = sqlx::query!(
        r#"
            SELECT
            g.id as group_chat_room_id,
            g.group_name as group_name,
            g.group_image as group_image
            FROM
                group_chat_room as g
                INNER JOIN
                    group_member as gm
                ON  g.id = gm.group_chat_room_id
            WHERE
                gm.user_id = ?
            AND gm.leave_flag = FALSE
            AND g.group_name = ?
            AND g.delete_flag = FALSE
        "#,
        user_id,
        search_text
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let mut result:Vec<SearchNameGroupListResult> = vec![];

    for row in &search_name_group_list {
        let group = SearchNameGroupListResult {
            group_chat_room_id: row.group_chat_room_id.to_string(),
            group_name: row.group_name.to_string(),
            group_image:  match &row.group_image {
                Some(group_image) => Some(group_image.to_string()),
                None => None
            },
          };
          result.push(group);
    }

    Ok(result)
}

/*
  ユーザが所属するグループ一覧
*/
// handler
#[derive(Debug, Deserialize, Serialize)]
struct FetchGroupListPath {
    user_id: String,
}
async fn handler_fetch_group_list(Path(path): Path<FetchGroupListPath>) -> Json<Value> {
    let user_id = path.user_id;

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let group_list = fetch_group_list(&pool, &user_id).await.unwrap();
    Json(json!({ "groups": group_list }))
}

#[derive(Debug, Deserialize, Serialize)]
struct FetchGroupListResult {
    group_chat_room_id: String,
    group_name: String,
    group_image: Option<String>
}

// SQL実行部分(Groups)
async fn fetch_group_list(pool: &MySqlPool, user_id:&str) -> anyhow::Result<Vec<FetchGroupListResult>>{
    let group_list = sqlx::query!(
        r#"
            SELECT
                g.id as group_chat_room_id,
                g.group_name as group_name,
                g.group_image as group_image
            FROM
                group_chat_room as g
                INNER JOIN
                    group_member as gm
                ON  g.id = gm.group_chat_room_id
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

    let mut result:Vec<FetchGroupListResult> = vec![];

    for row in &group_list {
        let group = FetchGroupListResult {
            group_chat_room_id: row.group_chat_room_id.to_string(),
            group_name: row.group_name.to_string(),
            group_image:  match &row.group_image {
                Some(group_image) => Some(group_image.to_string()),
                None => None
            },
          };
          result.push(group);
    }
    Ok(result)
}

/*
  グループからの脱退
*/
// handler
// pathはクエリの一部
// paramsは?以降
#[derive(Debug, Deserialize, Serialize)]
struct LeaveGroupPath {
    user_id: String,
}
async fn handler_leave_group(
    Path(path): Path<LeaveGroupPath>,
    body_json: Json<Value>
) -> () {
    let user_id = path.user_id;

    // group_chat_room_idの取得
    let group_chat_room_id = body_json.0.get("group_chat_room_id")
    .unwrap() // group_chat_room_idはNOT NULLなのでunwrap()使える
    .as_str()
    .unwrap();

    // friends
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    leave_group(&pool, &user_id, &group_chat_room_id).await.unwrap();
}
// SQL実行部分
async fn leave_group(pool: &MySqlPool, user_id:&str, group_chat_room_id: &str) -> anyhow::Result<()>{
    sqlx::query!(
        r#"
            UPDATE
                group_member
            SET
                leave_flag = TRUE
            WHERE
                group_chat_room_id = ?
            AND user_id = ?
        "#,
        group_chat_room_id,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(())
}

/*
  グループ追加
*/
// handler

#[derive(Debug, Deserialize, Serialize)]
struct AddGroupJson {
    group_image: Option<String>,
    group_name: String,
    group_member_user_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AddGroupResult {
    group_image: Option<String>,
    group_name: String,
    group_chat_room_id : String
}

// handler
async fn handler_add_group(
    body_json: Json<AddGroupJson>
) -> Json<Value> {
    // group_imageの取得
    let group_image = match &body_json.group_image{
        Some(group_image) => Some(group_image),
        None => None
    };

    // group_nameの取得
    let group_name = &body_json.group_name;

    // group_member_user_idsの取得
    let group_member_user_ids = &body_json.group_member_user_ids;

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let result = add_group(&pool, group_image, &group_name, &group_member_user_ids).await.unwrap();
    Json(json!({ "group_info": result }))
}

// SQL実行部分
async fn add_group(pool: &MySqlPool, group_image:Option<&String>, group_name: &str, group_member_user_ids: &Vec<String>) -> anyhow::Result<AddGroupResult> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    // group_chat_roomテーブルの追加
    let group_chat_room_id = sqlx::query!(
        r#"
            INSERT INTO group_chat_room ( group_name, group_image, created_at )
            VALUES ( ?, ? , ? )
        "#,
        &group_name,
        &group_image,
        &now
    )
    .execute(pool)
    .await
    .unwrap()
    .last_insert_id();

    // group_memberテーブルの追加
    for user_id in group_member_user_ids {
        sqlx::query!(
            r#"
                INSERT INTO group_member ( group_chat_room_id, user_id, entry_date )
                VALUES ( ?, ? , ? )
            "#,
            &group_chat_room_id,
            user_id,
            &now
        )
        .execute(pool)
        .await
        .unwrap();
    }

    let result = AddGroupResult {
        // group_image: match group_image {
        //     Some(group_image) => Some(group_image),
        //     None => None
        // },
        group_image: match group_image{
            Some(group_image) => Some(group_image.clone()), // &StringをStringにするには、.clone()を行う
            None => None
        },
        group_name: group_name.to_string(),
        group_chat_room_id : group_chat_room_id.to_string()
    };

    // group_chat_room、group_memberテーブルのレコード削除SQL
    // DELETE FROM group_member WHERE group_chat_room_id = 【対象id】;
    // DELETE FROM group_chat_room WHERE id = 【対象group_chat_room_id】;
    
    Ok(result)
}

/*
  ユーザーの所属するグループ数取得
*/
// handler
#[derive(Debug, Deserialize, Serialize)]
struct FetchGroupCountPath {
    user_id: String,
}
async fn handler_fetch_group_count(Path(path): Path<FetchGroupCountPath>) -> Json<Value> {
    let user_id = path.user_id;

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let group_count = fetch_group_count(&pool, &user_id).await.unwrap();
    Json(json!({ "group_count": group_count }))
}

// SQL実行部分
async fn fetch_group_count(pool: &MySqlPool, user_id:&str) -> anyhow::Result<i64>{
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

/*
   ユーザの友達数取得
*/
// handler
#[derive(Debug, Deserialize, Serialize)]
struct FetchFriendCountPath {
    user_id: String,
}
async fn handler_fetch_friend_count(Path(path): Path<FetchFriendCountPath>) -> Json<Value> {
    let user_id = path.user_id;

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let friend_count = fetch_friend_count(&pool, &user_id).await.unwrap();
    Json(json!({ "friend_count": friend_count }))
}

// SQL実行部分
async fn fetch_friend_count(pool: &MySqlPool, user_id:&str) -> anyhow::Result<i64>{
    let friend_count = sqlx::query!(
        r#"
            SELECT
                COUNT(*) as friend_count
            FROM
                follow
            WHERE
                from_user_id = ?
            "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    Ok(friend_count[0].friend_count)
}

/*
  ユーザーの友達一覧取得
*/
// handler
#[derive(Debug, Deserialize, Serialize)]
struct FetchFriendListPath {
    user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FetchFriendListResult {
    direct_chat_room_id: u64,
    friend_use_id: String,
    friend_profile_image: Option<String>,
    friend_nickname: Option<String>
}

async fn handler_fetch_friend_list(Path(path): Path<FetchFriendListPath>) -> Json<Value> {
    let user_id = path.user_id;

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let friend_list = fetch_friend_list(&pool, &user_id).await.unwrap();
    Json(json!({ "friend_list": friend_list }))
}

// SQL実行部分
async fn fetch_friend_list(pool: &MySqlPool, user_id:&str) -> anyhow::Result<Vec<FetchFriendListResult>>{
    let friend_list = sqlx::query!(
        r#"
            SELECT
                f.direct_chat_room_id as direct_chat_room_id,
                u.id as friend_use_id,
                u.profile_image as friend_profile_image,
                u.nickname as friend_nickname
            FROM
                user as u
                INNER JOIN
                    follow as f
                ON  u.id = f.to_user_id
            WHERE
                u.id IN(
                    SELECT
                        f.to_user_id
                    FROM
                        follow as f
                    WHERE
                        f.from_user_id = ?
                )
            "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let mut result:Vec<FetchFriendListResult> = vec![];

    for row in &friend_list {
        let friend = FetchFriendListResult {
            direct_chat_room_id: row.direct_chat_room_id,
            friend_use_id: row.friend_use_id.to_string(),
            friend_profile_image:  match &row.friend_profile_image {
                Some(friend_profile_image) => Some(friend_profile_image.to_string()),
                None => None
            },
            friend_nickname:  match &row.friend_nickname {
                Some(friend_nickname) => Some(friend_nickname.to_string()),
                None => None
            },
          };
          result.push(friend);
    }
    Ok(result)
}

/*
  友達追加
*/

#[derive(Debug, Deserialize, Serialize)]
struct AddFriendPath {
    user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AddFriendJson {
    friend_user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AddFriendResult {
    direct_chat_room_id: String,
    friend_use_id: String,
    friend_profile_image: Option<String>,
    friend_nickname: Option<String>
}

// handler
async fn handler_add_friend(
    Path(path): Path<AddFriendPath>,
    body_json: Json<AddFriendJson>
) -> Json<Value> {
    // own_user_idの取得
    let user_id = path.user_id;

    // friend_user_idの取得
    let friend_user_id = &body_json.friend_user_id;

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let result = add_friend(&pool, &user_id, &friend_user_id).await.unwrap();
    Json(json!({ "group_info": result }))
}

// SQL実行部分
async fn add_friend(pool: &MySqlPool, user_id:&str, friend_user_id: &str) -> anyhow::Result<AddFriendResult> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    // followテーブルで、from_user_idが相手、to_user_idが自分のdirect_chat_room_idがあるか確認
    let result = sqlx::query!(
        r#"
            SELECT
                *
            FROM
                follow
            WHERE
                to_user_id = ?
            AND from_user_id = ?
            "#,
        user_id,
        friend_user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    if result.len() == 0 {
        // まだdirect_chat_roomテーブルが存在しない場合
        // ①direct_chat_roomテーブルの作成
        let direct_chat_room_id = sqlx::query!(
            r#"
                INSERT INTO direct_chat_room ( created_at )
                VALUES ( ? )
            "#,
            &now
        )
        .execute(pool)
        .await
        .unwrap()
        .last_insert_id();

        // direct_chat_room_idをu64からStringに変換
        let direct_chat_room_id = &direct_chat_room_id.to_string();

        // ②direct_memberテーブルの作成
        // 自分のレコード追加
        sqlx::query!(
            r#"
                INSERT INTO direct_member ( direct_chat_room_id, user_id, entry_date, last_read_time )
                VALUES ( ?, ?, ?, ? )
            "#,
            &direct_chat_room_id,
            &user_id,
            &now,
            &now
        )
        .execute(pool)
        .await
        .unwrap();

        // 友達のレコード追加
        sqlx::query!(
            r#"
                INSERT INTO direct_member ( direct_chat_room_id, user_id, entry_date, last_read_time )
                VALUES ( ?, ?, ?, ? )
            "#,
            &direct_chat_room_id,
            &friend_user_id,
            &now,
            &now
        )
        .execute(pool)
        .await
        .unwrap();

        // ③followテーブルの作成
        sqlx::query!(
            r#"
                INSERT INTO follow ( to_user_id, from_user_id, direct_chat_room_id, created_at )
                VALUES ( ?, ?, ?, ? )
            "#,
            &friend_user_id,
            &user_id,
            &direct_chat_room_id,
            &now
        )
        .execute(pool)
        .await
        .unwrap();

        // 友達情報の取得
        let result = add_friend_result(&pool, &friend_user_id, &direct_chat_room_id).await.unwrap();
        Ok(result)

    } else {
        let direct_chat_room_id = &result[0].direct_chat_room_id.to_string();
        // 既にdirect_chat_roomテーブルが存在する場合
        // 既にfollowテーブルが作成されていないか確認(APIテスターでの誤り防止)
        let result = sqlx::query!(
            r#"
                SELECT
                    *
                FROM
                    follow
                WHERE
                    to_user_id = ?
                AND from_user_id = ?
                "#,
            &friend_user_id,
            &user_id
        )
        .fetch_all(pool)
        .await
        .unwrap();

        if result.len() == 0 {
            // まだfollowテーブルが生成されていない場合
            // ①followテーブルの作成
            sqlx::query!(
                r#"
                    INSERT INTO follow ( to_user_id, from_user_id, direct_chat_room_id, created_at )
                    VALUES ( ?, ?, ?, ? )
                "#,
                &friend_user_id,
                &user_id,
                &direct_chat_room_id,
                &now
            )
            .execute(pool)
            .await
            .unwrap();
        } else {
            // すでにfollowテーブルが生成されている場合(APIテスターでの重複登録の場合)
            // 何もしない
        }
        // 友達情報の取得
        let result = add_friend_result(&pool, &friend_user_id, &direct_chat_room_id).await.unwrap();
        Ok(result)
    }

    // group_chat_room、group_memberテーブルのレコード削除SQL
    // DELETE FROM direct_member WHERE direct_chat_room_id = 【対象id】;
    // DELETE FROM direct_chat_room WHERE id = 【対象group_chat_room_id】;

}

async fn add_friend_result(pool: &MySqlPool,friend_user_id: &str, direct_chat_room_id: &str) -> anyhow::Result<AddFriendResult> {
    let friend_info = sqlx::query!(
        r#"
            SELECT
                *
            FROM
                user
            WHERE
                id = ?
            "#,
        &friend_user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let result = AddFriendResult {
        direct_chat_room_id: direct_chat_room_id.to_string(),
        friend_use_id: friend_user_id.to_string(),
        friend_profile_image:  match &friend_info[0].profile_image {
            Some(friend_profile_image) => Some(friend_profile_image.to_string()),
            None => None
        },
        friend_nickname:  match &friend_info[0].nickname {
            Some(friend_nickname) => Some(friend_nickname.to_string()),
            None => None
        },
    };

    Ok(result)
}

/*
  ユーザーIDに紐づくニックネーム、プロフィール画像の取得
*/
// handler
#[derive(Debug, Deserialize, Serialize)]
struct FetchProfileByUserIdPath {
    user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FetchProfileByUserIdResult {
    user_id: String,
    nickname: Option<String>,
    profile_image: Option<String>,
    search_flag: bool
}

async fn handler_fetch_profile_by_user_id(Path(path): Path<FetchFriendListPath>) -> Json<Value> {
    let user_id = path.user_id;

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let profile = fetch_profile_by_user_id(&pool, &user_id).await.unwrap();
    Json(json!({ "friend_list": profile }))
}

// SQL実行部分
async fn fetch_profile_by_user_id(pool: &MySqlPool, user_id:&str) -> anyhow::Result<FetchProfileByUserIdResult>{
    let result = sqlx::query!(
        r#"
        SELECT
            nickname,
            profile_image,
            search_flag
        FROM
            user
        WHERE
            id = ?
            "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let profile_info = FetchProfileByUserIdResult {
        user_id: user_id.to_string(),
        nickname: match &result[0].nickname {
            Some(nickname) => Some(nickname.to_string()),
            None => None
        },
        profile_image: match &result[0].profile_image {
            Some(profile_image) => Some(profile_image.to_string()),
            None => None
        },
        search_flag: if result[0].search_flag == 1 { true } else { false }
    };

    Ok(profile_info)
}

/*
  プロフィールの更新
*/
// handler
#[derive(Debug, Deserialize, Serialize)]
struct UpdateProfilePath {
    user_id: String
}
#[derive(Debug, Deserialize, Serialize)]
struct UpdateProfileJson {
    nickname: Option<String>,
    profile_image: Option<String>,
    search_flag: Option<bool>
}

#[derive(Debug, Deserialize, Serialize)]
enum UpdateProfileResultEnum {
    UpdateNicknameResult {
        nickname: Option<String>,
    },
    UpdateProfileImageGResult {
        profile_image: Option<String>,
    },
    UpdateSearchFlagResult {
        search_flag: Option<bool>
    }    
}

// handler
async fn handler_update_profile(
    Path(path): Path<UpdateProfilePath>,
    body_json: Json<UpdateProfileJson>
) -> Json<Value> {
    // user_idの取得
    let user_id = path.user_id;

    // nicknameの取得
    let nickname = match &body_json.nickname{
        Some(nickname) => Some(nickname),
        None => None
    };

    // profile_imageの取得
    let profile_image = match &body_json.profile_image{
        Some(profile_image) => Some(profile_image),
        None => None
    };

    // search_flagの取得
    let search_flag = match &body_json.search_flag{
        Some(search_flag) => Some(search_flag),
        None => None
    };

    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let result = update_profile(&pool, &user_id, nickname, profile_image, search_flag).await.unwrap();
    Json(json!({ "group_info": result }))
}

// SQL実行部分
async fn update_profile(pool: &MySqlPool, user_id: &str, nickname:Option<&String>, profile_image:Option<&String>, search_flag:Option<&bool>) -> anyhow::Result<()> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    
    // nicknameの更新
    if let Some(_) = nickname {
        sqlx::query!(
            r#"
                UPDATE
                    user
                SET
                    nickname = ?,
                    updated_at = ?
                WHERE
                id = ?
            "#,
            nickname,
            now,
            user_id,
        )
        .execute(pool)
        .await?
        .rows_affected();
    }

    // profile_imageの更新
    if let Some(_) = profile_image {
        sqlx::query!(
            r#"
                UPDATE
                    user
                SET
                    profile_image = ?,
                    updated_at = ?
                WHERE
                    id = ?
            "#,
            profile_image,
            now,
            user_id
        )
        .execute(pool)
        .await?
        .rows_affected();
    }

    // search_flagの更新
    if let Some(_) = search_flag {
        sqlx::query!(
            r#"
                UPDATE
                    user
                SET
                    search_flag = ?,
                    updated_at = ?
                WHERE
                    id = ?
            "#,
            search_flag,
            now,
            user_id
        )
        .execute(pool)
        .await?
        .rows_affected();
    }
    
    Ok(())
}

/*
  ユーザーID検索にヒットしたユーザー情報(プロフィール画像、ニックネーム)
*/
#[derive(Debug, Deserialize, Serialize)]
struct FetchFriendInfoByUserIdPath {
    user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FetchFriendInfoByUserIdQuery {
    search_user_id: String,
}

// デフォルト値の取得
impl Default for FetchFriendInfoByUserIdQuery {
    fn default() -> Self {
        Self { search_user_id: String::from("") }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct FetchFriendInfoByUserIdResult {
    exist_user_id: bool,
    already_follow_requested: bool,
    friend_search_flag: bool,
    friend_use_id: Option<String>,
    friend_profile_image: Option<String>,
    friend_nickname: Option<String>
}

// handler
async fn handler_fetch_friend_info_by_friend_user_id(
    Path(path): Path<SearchNamePath>,
    query: Option<Query<FetchFriendInfoByUserIdQuery>>,
) -> Json<Value> {
    let user_id = path.user_id;
    // unwrap_or_default: Okの場合値を返し、Errの場合値の型のデフォルトを返す
    let Query(query) = query.unwrap_or_default();
    let search_user_id = query.search_user_id;

    // friends
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    let result = fetch_friend_info_by_friend_user_id(&pool, &user_id, &search_user_id).await.unwrap();
    
    Json(json!({ "result": result }))
}

// SQL実行部分(Friends)
async fn fetch_friend_info_by_friend_user_id(pool: &MySqlPool, user_id: &str, search_user_id: &str) -> anyhow::Result<FetchFriendInfoByUserIdResult> {
    // パターン
    // ①該当のユーザーIDが存在しない
    // ②該当のユーザーIDが存在する
        // 1. すでに友達である
        // 2. まだ友達ではない
            // (1) search_flagがtrue
            // (2) search_flagがfalse
    
    // 該当のユーザーIDが存在しているか
    let exist_user_id = sqlx::query!(
        r#"
            SELECT
                count(*) as exist_user_id
            FROM
                user
            WHERE
                id = ?
            "#,
            search_user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let exist_user_id:bool = if  exist_user_id[0].exist_user_id == 1 { true } else { false };
    println!("{:?} exist_user_id", exist_user_id);

    if !exist_user_id {
        // 該当のユーザーIDが存在していない場合
        let result = FetchFriendInfoByUserIdResult {
            exist_user_id: false,
            already_follow_requested: false,
            friend_search_flag: false,
            friend_use_id: None,
            friend_profile_image: None,
            friend_nickname: None
        };
        return Ok(result)
    } else {
        // 該当のユーザーIDが存在している場合
        // search_user_idのユーザーのsearch_flag
        let search_friend_user_info = sqlx::query!(
            r#"
                SELECT
                    *
                FROM
                    user
                WHERE
                    id = ?
                "#,
                search_user_id
        )
        .fetch_all(pool)
        .await
        .unwrap();

        let search_flag:bool = if   search_friend_user_info[0].search_flag == 1 { true } else { false };
        println!("{:?}", search_flag);
        // 既に友達になっているかのチェック
        let result = sqlx::query!(
            r#"
                SELECT
                    count(*) as already_follow_requested
                FROM
                    follow
                WHERE
                    to_user_id = ?
                AND from_user_id = ?
                "#,
                search_user_id,
                user_id
        )
        .fetch_all(pool)
        .await
        .unwrap();

        let already_follow_requested = if result[0].already_follow_requested == 1 { true } else { false };
        println!("{:?} already_follow_requested", already_follow_requested);

        if !already_follow_requested {
            // まだ友達になっていない場合
            if search_flag {
                // 該当search_user_idのユーザーのsearch_flagがtrueの場合、友達情報を出力
                let result = FetchFriendInfoByUserIdResult {
                    exist_user_id: exist_user_id,
                    already_follow_requested: already_follow_requested,
                    friend_search_flag: search_flag,
                    friend_use_id: Some(search_friend_user_info[0].id.clone()),
                    friend_profile_image: match &search_friend_user_info[0].profile_image {
                        Some(profile_image) => Some(profile_image.to_string()),
                        None => None
                    },
                    friend_nickname: match &search_friend_user_info[0].nickname {
                        Some(friend_nickname) => Some(friend_nickname.to_string()),
                        None => None
                    },
                };
                return Ok(result)
            } else {
                // 該当search_user_idのユーザーのsearch_flagがfalseの場合、友達情報は出力しない
                let result = FetchFriendInfoByUserIdResult {
                    exist_user_id: exist_user_id,
                    already_follow_requested: already_follow_requested,
                    friend_use_id: Some(search_friend_user_info[0].id.clone()),
                    friend_search_flag: search_flag,
                    friend_profile_image: match &search_friend_user_info[0].profile_image {
                        Some(profile_image) => Some(profile_image.to_string()),
                        None => None
                    },
                    friend_nickname: match &search_friend_user_info[0].nickname {
                        Some(friend_nickname) => Some(friend_nickname.to_string()),
                        None => None
                    },
                };
                return Ok(result)
            }
        } else {
            // 既に友達である場合
            let result = FetchFriendInfoByUserIdResult {
                exist_user_id: exist_user_id,
                already_follow_requested: already_follow_requested,
                friend_use_id: Some(search_friend_user_info[0].id.clone()),
                friend_search_flag: search_flag,
                friend_profile_image: match &search_friend_user_info[0].profile_image {
                    Some(profile_image) => Some(profile_image.to_string()),
                    None => None
                },
                friend_nickname: match &search_friend_user_info[0].nickname {
                    Some(friend_nickname) => Some(friend_nickname.to_string()),
                    None => None
                },
            };
            return Ok(result)

        }
    }
}