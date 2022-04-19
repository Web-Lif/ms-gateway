use actix_web::{web, post, Responder, http::StatusCode};
use base64::encode;
use chrono::{DateTime, Utc};
use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey, PaddingScheme, Hash};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Sha512, Digest};
use chrono::serde::ts_seconds_option;

use crate::config::app_data;

#[derive(Deserialize, Serialize)]
struct LoginParam {
    username: String,
    password: String
}

#[derive(sqlx::FromRow)]
pub struct MSCoreUser {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub email: String,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>
}


#[derive(Serialize)]
struct Token {
    /** 用户 id */
    id: i64,
    /** 用户名 */
    username: String,
    /** 邮箱地址 */
    email: String,
    /** 创建时间 */
    #[serde(with = "ts_seconds_option")]
    create_at: Option<DateTime<Utc>>,
}

const RSA_2048_PRIV_PEM: &str = include_str!("../../resources/private_key.pem");

/**
 * 用户登入, 通过帐号和密码进行登入
 */
#[post("/auth/login")]
async fn login(param: web::Json<LoginParam>, app: web::Data<app_data::AppGlobalData>) -> impl Responder {
    let rows: Vec<MSCoreUser> = sqlx::query_as::<_, MSCoreUser>(
        "select id,  from ms_core_user where username = ? and password = ?"
    )
    .bind(&param.username)
    .bind(&param.password)
    .fetch_all(&app.pool).await.unwrap();
    if rows.len() > 0 {
        let user = &rows[0];
        let key = RsaPrivateKey::from_pkcs8_pem(RSA_2048_PRIV_PEM).unwrap();

        let token_str = serde_json::to_string(&Token {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            create_at: Some(Utc::now())
        }).unwrap();
    
        let mut hasher = Sha512::new();
        hasher.update(&token_str);
        let hash = hasher.finalize();

        let sign_result = key.sign(
            PaddingScheme::PKCS1v15Sign {hash: Some(Hash::SHA2_512) }, &hash[..]);
        let sign = encode(sign_result.unwrap());

        return (
            web::Json(json!({
                "token": format!("{}.{}", &token_str, sign),
            })),
            StatusCode::OK
        )
    }
    (
        web::Json(json!({ "message": "帐号或则密码不正确"})),
        StatusCode::BAD_REQUEST
    )
}


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}